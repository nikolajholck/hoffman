extern crate hoffman;
extern crate serde_json;

use hoffman::*;
use hoffman::four::*;
use std::time::Instant;
use std::collections::HashMap;
use std::iter::repeat;

use serde_json::{Value};
use std::fs::File;

fn main() {
    let bricks = [
        [8, 9, 10, 12],  // Wide
        [10, 12, 13, 14] // Narrow
    ];

    println!("Bricks: {:?}", bricks);

    assert!(bricks.iter().all(|&brick| list_has_unique_sums(&brick)), "Brick doesn't have unique sums.");

    println!("Will determine packings.");
    let now = Instant::now();
    let packings = backtrack_tesseracts(&bricks);

    for (i, &(positions, sizes)) in packings.iter().enumerate() {
        let name = format!("4D Packing {}", i);
        tesseract::plot(&positions, &sizes, &bricks[0], &name);
        tesseract::export(&positions, &sizes, &bricks[0], &name);
    }

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

fn permute(brick: &Brick, permutation: &[usize]) -> Brick {
    let mut result: Brick = [0; N];
    for (i, &res) in permutation.iter().enumerate() {
        result[i] = brick[res];
    }
    result
}

fn backtrack_tesseracts(bricks: &[Brick]) -> Vec<(tesseract::Tesseract, tesseract::Tesseract)> {
    let mut packings: Vec<Packing> = bricks.iter().map(|brick| Packing::new(brick)).collect();

    let max_tries = N * (N - 1) * (N - 2) * (N - 3);
    let coords = tesseract::make_coords([N; N]);
    println!("Coords: {:?}", coords.len());
    let mut solutions = Vec::new();

    let mut records = [[[[0; N]; N]; N]; N];
    let mut i: usize = 0;
    let mut iteration: usize = 0;
    let mut successes: usize = 0;

    let answer = {
        let file = File::open("res/packing-4d.json").unwrap();
        let json: Value = serde_json::from_reader(file).unwrap();
        let bs: Vec<Value> = json["bricks"].as_array().unwrap().to_vec();
        let perms: Vec<tesseract::Coord> = bs.iter().map(|ref b| {
            let map = b.as_object().unwrap();
            let perm = &map["size"];
            let mut res: [usize; N] = [0; N];
            for j in 0..N {
                res[j] = perm[j].as_u64().unwrap() as usize;
            }
            res
        }).collect();
        perms
    };

    for k in 0..N * N * N * N - 94 {
        let coord = coords[k];
        for (j, packing) in packings.iter_mut().enumerate() {
            let orientation = permute(&bricks[j], &answer[k]);
            let b = Point4D {
                x: orientation[0],
                y: orientation[1],
                z: orientation[2],
                w: orientation[3]
            };
            let orientation_index = packing.bricks.iter().position(|v| *v == b).unwrap();
            packing.place(&coord, orientation_index);
            assert!(packing.is_valid(&coord));
        }
        let (x, y, z, w) = (coord[0], coord[1], coord[2], coord[3]);
        records[x][y][z][w] = max_tries;
        i = i + 1;
    }
    println!("Configured.");

    loop {
        iteration += 1;
        if iteration % 10_000_000 == 0 {
            println!("Iteration {:?}, successes: {:?}, records:", iteration, successes);
            /*let name = format!("4D Packing {}", iteration);
            tesseract::plot(&packings[0].positions, &packings[0].sizes, &bricks[0], &name);*/
        }

        let coord = coords[i];
        let (x, y, z, w) = (coord[0], coord[1], coord[2], coord[3]);

        if records[x][y][z][w] < max_tries { // We'll try placing a brick.
            for packing in &mut packings {
                packing.place(&coord, records[x][y][z][w]);
            }

            records[x][y][z][w] += 1; // Register that this rotation has been tried.

            if packings.iter().all(|packing| packing.is_valid(&coord)) {
                if i == N * N * N * N - 1 { // We have successfully placed all bricks.
                    successes += 1;
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
    panic!("Total found: {:?}", successes);
    solutions
}
