extern crate hoffman;

use std::time::Instant;
use hoffman::*;

const N: usize = 4;
const M: usize = 3;

fn main() {
    let dimension_tuples = vec!(
        vec!(8, 9, 10, 12),  // Wide
        vec!(10, 12, 13, 14) // Narrow
    );

    println!("Dimension tuples: {:?}", dimension_tuples);

    assert!(dimension_tuples.iter().all(|dimension_tuple| utils::list_has_unique_sums(&dimension_tuple)), "Dimension tuple does not have unique sums.");

    println!("Will determine packings.");
    let now = Instant::now();
    backtrack_cubes(&dimension_tuples);
    println!("Time spent making recipes: {:?} s", now.elapsed().as_secs());
}

fn backtrack_cubes(dimension_tuples: &Vec<DimensionTuple>) {
    let mut recipe_builder = RecipeBuilder::new(N, M, dimension_tuples.clone());

    let max_tries = N * (N - 1) * (N - 2);
    let coords = recipe_builder.get_recipe().map.coords().clone();
    println!("Coords: {:?}", coords.len());

    let mut records = [[[0; N]; N]; N];
    let mut i: usize = 0;
    let mut iteration: usize = 0;
    let mut successes: usize = 0;
    let now = Instant::now();

    let indices: Vec<usize> = (0..N).collect();
    let perms = combinatorics::permutations(&indices.as_slice(), M);

    loop {
        iteration += 1;
        if iteration % 10_000_000 == 0 {
            println!("Have spent {} seconds at iteration {}.", now.elapsed().as_secs(), iteration);
            println!("Hyper-rectangles placed: {}.", i);
            println!("Current Records:");
            for level in &records {
                for row in level {
                    println!("{:?}", row);
                }
            }
            println!();
        }

        let coord = &coords[i];
        let (x, y, z) = (coord[0], coord[1], coord[2]);

        if records[x][y][z] < max_tries { // We'll try placing a brick.
            recipe_builder.insert(coord, &perms[records[x][y][z]]);
            records[x][y][z] += 1; // Register that this rotation has been tried.

            if recipe_builder.is_valid(coord) {
                if i == N * N * N - 1 { // We have successfully placed all bricks.
                    successes += 1;
                    let name = format!("cube-4d-{}", successes);
                    recipe_builder.get_recipe().save_json(&String::from("cubes"), &name);
                    if successes >= 1000 { return }
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
            records[x][y][z] = 0; // Reset tries.
            i -= 1; // Backtrack.
        }
        recipe_builder.remove(&coords[i]);
    }
    println!("Total iterations {:?}", iteration);
    panic!("Too good to be true!");
}
