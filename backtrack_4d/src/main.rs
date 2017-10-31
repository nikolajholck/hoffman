extern crate hoffman;

use hoffman::*;
use hoffman::four::*;
use std::time::Instant;
use std::collections::HashMap;

fn main() {
    let brick: Brick = [57, 59, 62, 63];
    println!("Brick: {:?}", brick);

    let comparator = Comparator::constructor(&brick);

    println!("Will determine packings.");
    let now = Instant::now();
    let (positions, sizes) = backtrack(&brick, &comparator);
    let name = format!("4D Packing final");
    tesseract::plot(&positions, &sizes, &brick, &name);
    println!("Positions: {:?}", positions);
    println!("Sizes: {:?}", sizes);

    println!("Time spent making packing: {:?} s", now.elapsed().as_secs());

}

fn backtrack(brick: &Brick, comparator: &Comparator) -> (tesseract::Tesseract, tesseract::Tesseract) {
    let mut type_counts: Vec<Vec<HashMap<IntType, usize>>> = Vec::new();
    for _dim in 0..N {
        let mut dim_counts = Vec::new();
        for _level in 0..N {
            dim_counts.push(HashMap::with_capacity(N));
        }
        type_counts.push(dim_counts);
    }
    println!("{:?}", type_counts);

    let mut rotations = [Point4D::ZERO; N * (N - 1) * (N - 2) * (N - 3)];
    for (i, permutation) in permutations(brick, N).iter().enumerate() {
        rotations[i] = Point4D { x: permutation[0], y: permutation[1], z: permutation[2], w: permutation[3] };
    }
    println!("Rotations: {:?}", rotations.len());

    let coords = tesseract::make_coords();
    println!("Coords: {:?}", coords.len());

    let mut positions = [[[[Point4D::ZERO; N]; N]; N]; N];
    let mut sizes = [[[[Point4D::ZERO; N]; N]; N]; N];
    let mut records = [[[[0; N]; N]; N]; N];

    let mut i: usize = 0;
    let mut iteration: usize = 0;
    //let mut successes: usize = 0;

    loop {
        if i == N * N * N * N { // We have successfully placed all bricks.
            println!("Iterations: {:?}", iteration);
            println!("Records: {:?}", records);
            return (positions, sizes);
            //successes += 1;
            i = i - 1; // Carry on.
            let p = coords[i];
            let (px, py, pz, pw) = (p[0], p[1], p[2], p[3]);
            decrement_type_count(&mut type_counts, &sizes[px][py][pz][pw], &p);
            sizes[px][py][pz][pw] = Point4D::ZERO; // Remove brick from sizes.
            positions[px][py][pz][pw] = Point4D::ZERO; // Remove brick from positions.
        }

        let coord = coords[i];
        let (x, y, z, w) = (coord[0], coord[1], coord[2], coord[3]);
        let max_tries = rotations.len();

        if records[x][y][z][w] == max_tries { // We have tried all rotations at this coord.
            if i == 0 {
                // There aren't any more possibilities.
                println!("We have tried everything! :)");
                break;
            }
            records[x][y][z][w] = 0; // Reset tries.
            i -= 1; // Backtrack.
            let p = coords[i];
            let (px, py, pz, pw) = (p[0], p[1], p[2], p[3]);
            decrement_type_count(&mut type_counts, &sizes[px][py][pz][pw], &p);
            sizes[px][py][pz][pw] = Point4D::ZERO; // Remove brick from sizes.
            positions[px][py][pz][pw] = Point4D::ZERO; // Remove brick from positions.
        } else { // We'll try placing a brick.
            let next_brick = rotations[records[x][y][z][w]];
            records[x][y][z][w] += 1; // Register that this rotation has been tried.
            sizes[x][y][z][w] = next_brick;
            tesseract::position_brick(&mut positions, &sizes, &coord);
            increment_type_count(&mut type_counts, &next_brick, &coord);
            if tesseract::is_brick_valid(&positions, &sizes, &coord, &comparator)
            && validate_type_count(&type_counts, &next_brick, &coord)
            && !tesseract::makes_sharp_corner(&positions, &sizes, &coord, &comparator) {
                i += 1; // Go to next coord.
            } else {
                sizes[x][y][z][w] = Point4D::ZERO; // Remove brick from sizes.
                positions[x][y][z][w] = Point4D::ZERO; // Remove brick from positions.
                decrement_type_count(&mut type_counts, &next_brick, &coord);
            }
        }

        iteration += 1;
        if iteration % 10_000_000 == 0 {
            println!("Status: Iteration {:?}, i: {:?}, records:", iteration, i);
            for row in &records[1] {
                println!("{:?}", row);
            }
            let name = format!("4D Packing {}", iteration);
            tesseract::plot(&positions, &sizes, brick, &name);
        }
    }
    println!("Total iterations {:?}", iteration);
    panic!("Too good to be true!");
}

fn validate_type_count(counts: &Vec<Vec<HashMap<IntType, usize>>>, brick: &Point4D, coord: &[usize; N]) -> bool {
    coord.iter().enumerate().all(|(i, &v)| counts[i][v][&brick[i]] <= N * N)
}

fn decrement_type_count(counts: &mut Vec<Vec<HashMap<IntType, usize>>>, brick: &Point4D, coord: &[usize; N]) {
    for i in 0..N {
        let count = counts[i][coord[i]].entry(brick[i]).or_insert(0);
        *count -= 1;
    }
}

fn increment_type_count(counts: &mut Vec<Vec<HashMap<IntType, usize>>>, brick: &Point4D, coord: &[usize; N]) {
    for i in 0..N {
        let count = counts[i][coord[i]].entry(brick[i]).or_insert(0);
        *count += 1;
    }
}
