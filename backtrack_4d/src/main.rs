extern crate hoffman;

use hoffman::*;
use hoffman::four::*;
use std::time::Instant;
use std::collections::HashMap;
use std::iter::repeat;

fn main() {
    let bricks = [
        [57, 59, 62, 63], // Narrow
        [53, 54, 57, 59]  // Wide
    ];

    println!("Bricks: {:?}", bricks);

    assert!(bricks.iter().all(|&brick| list_has_unique_sums(&brick)), "Brick doesn't have unique sums.");

    println!("Will determine packings.");
    let now = Instant::now();
    let packings = backtrack_tesseracts(&bricks);
    let (positions, sizes) = packings[0];
    let name = format!("4D Packing final");
    tesseract::plot(&positions, &sizes, &bricks[0], &name);
    println!("Positions: {:?}", positions);
    println!("Sizes: {:?}", sizes);

    println!("Time spent making packing: {:?} s", now.elapsed().as_secs());

}

struct Packing {
    positions: tesseract::Tesseract,
    sizes: tesseract::Tesseract,
    bricks: Vec<Point4D>,
    type_counts: Vec<Vec<HashMap<IntType, usize>>>,
}

impl Packing {
    fn new(brick: &Brick) -> Packing {
        Packing {
            positions: [[[[Point4D::ZERO; N]; N]; N]; N],
            sizes: [[[[Point4D::ZERO; N]; N]; N]; N],
            bricks: permutations(brick, N).iter().map(|permutation| {
                Point4D { x: permutation[0], y: permutation[1], z: permutation[2], w: permutation[3] }
            }).collect(),
            type_counts: repeat(
                repeat(HashMap::with_capacity(N)).take(N).collect() // For each level.
            ).take(N).collect() // For each dimension.
        }
    }

    fn place(&mut self, coord: &tesseract::Coord, brick_index: usize) {
        let (x, y, z, w) = (coord[0], coord[1], coord[2], coord[3]);
        let brick = self.bricks[brick_index];
        self.increment_type_count(&brick, &coord);
        self.sizes[x][y][z][w] = brick;
        tesseract::position_brick(&mut self.positions, &self.sizes, &coord);
    }

    fn remove(&mut self, coord: &tesseract::Coord) {
        let (x, y, z, w) = (coord[0], coord[1], coord[2], coord[3]);
        let brick = self.sizes[x][y][z][w];
        self.decrement_type_count(&brick, &coord);
        self.sizes[x][y][z][w] = Point4D::ZERO; // Remove brick from sizes.
        self.positions[x][y][z][w] = Point4D::ZERO; // Remove brick from positions.
    }

    fn is_valid(&self, coord: &tesseract::Coord) -> bool {
        tesseract::is_brick_valid(&self.positions, &self.sizes, &coord)
        && self.validate_type_count(&coord)
        && !tesseract::makes_sharp_corner(&self.positions, &self.sizes, &coord)
    }

    fn validate_type_count(&self, coord: &tesseract::Coord) -> bool {
        coord.iter().enumerate().all(|(i, &v)| self.type_counts[i][v].values().max().unwrap() <= &(N * N))
    }

    fn decrement_type_count(&mut self, brick: &Point4D, coord: &tesseract::Coord) {
        for i in 0..N {
            let count = self.type_counts[i][coord[i]].entry(brick[i]).or_insert(0);
            *count -= 1;
        }
    }

    fn increment_type_count(&mut self, brick: &Point4D, coord: &tesseract::Coord) {
        for i in 0..N {
            let count = self.type_counts[i][coord[i]].entry(brick[i]).or_insert(0);
            *count += 1;
        }
    }
}

fn backtrack_tesseracts(bricks: &[Brick]) -> Vec<(tesseract::Tesseract, tesseract::Tesseract)> {
    let mut packings: Vec<Packing> = bricks.iter().map(|brick| Packing::new(brick)).collect();
    let coords = tesseract::make_coords([N; N]);
    println!("Coords: {:?}", coords.len());
    let mut solutions = Vec::new();

    let mut records = [[[[0; N]; N]; N]; N];
    let mut i: usize = 0;
    let mut iteration: usize = 0;
    //let mut successes: usize = 0;

    loop {
        iteration += 1;
        if iteration % 10_000_000 == 0 {
            println!("Iteration {:?}, i: {:?}, records:", iteration, i);
            for row in &records[1] {
                println!("{:?}", row);
            }
            let name = format!("4D Packing {}", iteration);
            tesseract::plot(&packings[0].positions, &packings[0].sizes, &bricks[0], &name);
        }

        let coord = coords[i];
        let (x, y, z, w) = (coord[0], coord[1], coord[2], coord[3]);
        let max_tries = N * (N - 1) * (N - 2) * (N - 3);

        if records[x][y][z][w] < max_tries { // We'll try placing a brick.
            for packing in &mut packings {
                packing.place(&coord, records[x][y][z][w]);
            }

            records[x][y][z][w] += 1; // Register that this rotation has been tried.

            if packings.iter().all(|packing| packing.is_valid(&coord)) {
                if i == N * N * N * N - 1 { // We have successfully placed all bricks.
                    println!("Iterations: {:?}", iteration);
                    println!("Records: {:?}", records);
                    solutions.push((packings[0].positions, packings[0].sizes));
                    //successes += 1;
                    return solutions;
                } else {
                    i += 1; // Go to next coord.
                    continue;
                }
            }
        } else { // We have tried all rotations at this coord.
            if i == 0 {
                // There aren't any more possibilities.
                println!("We have tried everything.");
                break;
            }
            records[x][y][z][w] = 0; // Reset tries.
            i -= 1; // Backtrack.
        }
        for packing in &mut packings {
            packing.remove(&coords[i]);
        }
    }
    println!("Total iterations {:?}", iteration);
    panic!("Too good to be true!");
}
