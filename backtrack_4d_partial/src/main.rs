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
    backtrack_tesseracts(&dimension_tuples);

    println!("Time spent making recipes: {:?} s", now.elapsed().as_secs());
}

fn backtrack_tesseracts(dimension_tuples: &Vec<DimensionTuple>) {
    let mut recipe_builder = RecipeBuilder::new(N, N, dimension_tuples.clone());

    let max_tries = N * (N - 1) * (N - 2) * (N - 3);
    let coords = utils::make_coords(N, N);
    println!("Coords: {:?}", coords.len());

    let mut records = [[[[0; N]; N]; N]; N];
    let mut i: usize = 0;
    let mut iteration: usize = 0;
    let mut successes: usize = 0;

    let answer = Recipe::load_json(&String::from("res"), &String::from("packing-4d"));

    for k in 0..N * N * N * N - 94 {
        let coord = &coords[k];
        recipe_builder.insert(coord, answer.map.get(coord).unwrap());
        assert!(recipe_builder.is_valid(&coord));
        let (x, y, z, w) = (coord[0], coord[1], coord[2], coord[3]);
        records[x][y][z][w] = max_tries;
        i = i + 1;
    }
    println!("Configured.");

    let indices: Vec<usize> = (0..N).collect();
    let perms = combinatorics::permutations(&indices.as_slice(), N);

    loop {
        iteration += 1;
        if iteration % 10_000_000 == 0 {
            println!("Iteration {:?}, successes: {:?}, records:", iteration, successes);
            let name = format!("4D Packing {}", iteration);
            let recipe = recipe_builder.get_recipe();
            plot::plot_4d(&recipe, &dimension_tuples[0], &name);
            recipe.save_json(&String::from("tesseracts"), &name);
        }

        let coord = &coords[i];
        let (x, y, z, w) = (coord[0], coord[1], coord[2], coord[3]);

        if records[x][y][z][w] < max_tries { // We'll try placing a dimension_tuple.
            recipe_builder.insert(coord, &perms[records[x][y][z][w]]);
            records[x][y][z][w] += 1; // Register that this rotation has been tried.

            if recipe_builder.is_valid(coord) {
                if i == N * N * N * N - 1 { // We have successfully placed all dimension_tuples.
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
        recipe_builder.remove(&coords[i]);
    }
    println!("Total iterations {:?}", iteration);
    println!("Total found: {:?}", successes);
}
