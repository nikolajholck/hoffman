extern crate hoffman;

use std::time::Instant;
use hoffman::*;

const N: usize = 4;

fn main() {
    let dimension_tuples = vec!(
        vec!(8, 9, 10, 12),  // Wide
        vec!(10, 12, 13, 14) // Narrow
    );

    println!("Dimension tuples: {:?}", dimension_tuples);

    assert!(dimension_tuples.iter().all(|dimension_tuple| utils::list_has_unique_sums(&dimension_tuple)), "Dimension tuple does not have unique sums.");

    println!("Will determine packings.");
    let now = Instant::now();
    let recipe = backtrack_tesseracts(&dimension_tuples);
    let name = format!("4D packing found.");
    plot::plot_4d(&recipe, &dimension_tuples[0], &name);
    recipe.save_json(&String::from("tesseracts"), &name);
    println!("Time spent making recipe: {:?} s", now.elapsed().as_secs());
}

fn backtrack_tesseracts(dimension_tuples: &Vec<DimensionTuple>) -> Recipe {
    let mut recipe_builder = RecipeBuilder::new(N, N, dimension_tuples.clone());

    let max_tries = N * (N - 1) * (N - 2) * (N - 3);
    let coords = recipe_builder.get_recipe().map.coords().clone();
    println!("Coords: {:?}", coords.len());

    let mut records = [[[[0; N]; N]; N]; N];
    let mut i: usize = 0;
    let mut iteration: usize = 0;
    //let mut successes: usize = 0;
    let now = Instant::now();

    let indices: Vec<usize> = (0..N).collect();
    let perms = combinatorics::permutations(&indices.as_slice(), N);

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
            let name = format!("4D Packing {}", iteration);
            plot::plot_4d(&recipe_builder.get_recipe(), &dimension_tuples[0], &name);
        }

        let coord = &coords[i];
        let (x, y, z, w) = (coord[0], coord[1], coord[2], coord[3]);

        if records[x][y][z][w] < max_tries { // We'll try placing a brick.
            recipe_builder.insert(coord, &perms[records[x][y][z][w]]);
            records[x][y][z][w] += 1; // Register that this rotation has been tried.

            if recipe_builder.is_valid(coord) {
                if i == N * N * N * N - 1 { // We have successfully placed all bricks.
                    println!("Packing found!");
                    println!("Iterations: {:?}", iteration);
                    println!("Records: {:?}", records);
                    return recipe_builder.get_recipe().clone();
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
        recipe_builder.remove(&coords[i]);
    }
    println!("Total iterations {:?}", iteration);
    panic!("Too good to be true!");
}
