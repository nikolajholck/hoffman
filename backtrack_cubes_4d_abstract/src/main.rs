extern crate hoffman;

use hoffman::*;
use std::time::Instant;

const N: usize = 4;
const M: usize = 3;

fn main() {
    let dimension_tuples = vec!(
        vec!(8, 9, 10, 12),  // Wide
        vec!(10, 12, 13, 14) // Narrow
    );

    println!("Dimension tuples: {:?}", dimension_tuples);

    assert!(dimension_tuples.iter().all(|dimension_tuple| utils::list_has_unique_sums(&dimension_tuple)), "Dimension tuple does not have unique sums.");

    println!("Will determine kernels.");
    let now = Instant::now();
    let unique_kernels = backtrack_kernels(&dimension_tuples);
    println!("Unique kernel count: {:?}", unique_kernels.len());
    println!("Time spent making kernels: {:?} s", now.elapsed().as_secs());

    println!("Will determine number of unique cubes...");
    let now = Instant::now();
    let unique_cube_count: usize = unique_kernels.iter().map(|kernel| {
        backtrack_cubes(&dimension_tuples, &kernel)
    }).sum();
    println!("Total unique cube count: {:?}", unique_cube_count);
    println!("Time spent making cubes: {:?} s", now.elapsed().as_secs());
}

fn backtrack_kernels(dimension_tuples: &Vec<DimensionTuple>) -> Vec<Recipe> {

    let mut kernels = Vec::new();

    let indices: Vec<usize> = (0..N).collect();
    let perms = combinatorics::permutations(&indices.as_slice(), M);

    let coords: Vec<Coord> = utils::make_coords(M, M).iter().map(|coord| {
        coord.iter().map(|v| v + 1 ).collect::<Coord>()
    }).collect();
    //println!("Kernel coordinates: {:?}", coords);

    let mut recipe_builder = RecipeBuilder::new(N, M, dimension_tuples.clone());
    let mut records = [[[0; N]; N]; N];

    let max_tries = perms.len();

    let mut i: usize = 0;
    let mut iteration: usize = 0;
    let mut successes: usize = 0;

    loop {
        iteration += 1;
        if iteration % 100_000_000 == 0 {
            println!("Iteration {:?}, records: {:?}, successes: {:?}", iteration, records[0][0][0], successes);
        }

        let coord = &coords[i];
        let (x, y, z) = (coord[0], coord[1], coord[2]);

        if records[x][y][z] < max_tries { // We'll try placing a brick.
            recipe_builder.insert(coord, &perms[records[x][y][z]]); // Fetch next rotation and place brick.
            records[x][y][z] += 1; // Register that this rotation has been tried.
            if recipe_builder.satisfies_line_criterion(coord) {
                if i == M * M * M - 1 { // We have successfully placed bricks everywhere.
                    successes += 1;
                    kernels.push(recipe_builder.get_recipe().clone());
                    if kernels.len() > 0 {
                        break;
                    }
                } else {
                    i += 1; // Go to next coord.
                    continue;
                }
            }
        } else { // We have tried all rotations at this coord.
            if i == 0 {
                // There aren't any more possibilities.
                break;
            }
            records[x][y][z] = 0; // Reset tries.
            i -= 1 // Backtrack.
        }
        recipe_builder.remove(&coords[i]);
    }
    println!("Total kernels: {:?}", successes);
    println!("Kernel count including rotations and reflections: {:?}", kernels.len());
    kernels
}

fn backtrack_cubes(dimension_tuples: &Vec<DimensionTuple>, kernel: &Recipe) -> usize {
    let mut recipe_builder = RecipeBuilder::new(N, M, dimension_tuples.clone());

    let coords = utils::make_coords(N, M);
    //println!("Coordinates: {:?}", coords);

    let indices: Vec<usize> = (0..N).collect();
    let perms: Vec<Orientation> = combinatorics::permutations(&indices.as_slice(), M);

    let mut cubes = Vec::new();

    let mut records = [[[0; N]; N]; N];

    let mut i: usize = 0;
    let mut iteration: usize = 0;
    let mut successes: usize = 0;

    loop {
        iteration += 1;

        if iteration % 10_000_000 == 0 {
            println!("Status: Iteration {:?}, i: {:?}, successes: {:?}, record: {:?}", iteration, i, successes, records[0]);
        }

        let coord = &coords[i];
        let (x, y, z) = (coord[0], coord[1], coord[2]);
        let inside_kernel = kernel.map.contains_key(coord);
        let max_tries = if inside_kernel { 1 } else { N * (N - 1) * (N - 2) };

        if records[x][y][z] < max_tries { // We'll try placing a brick.
            let next_perm = if inside_kernel { // Fetch next rotation and place brick.
                &kernel.map.get(coord).unwrap()
            } else {
                &perms[records[x][y][z]]
            };

            recipe_builder.insert(coord, &next_perm);

            records[x][y][z] += 1; // Register that this rotation has been tried.

            if recipe_builder.is_valid(coord) {
                if i == N * N * N - 1 { // We have successfully placed all bricks.
                    successes += 1;
                    let recipe = recipe_builder.get_recipe().clone();
                    plot::plot_4d_cube(&recipe, &dimension_tuples[0], &format!("Cube at iteration {}", iteration));
                    cubes.push(recipe);
                } else {
                    i += 1; // Go to next coord.
                    continue;
                }
            }
        } else { // We have tried all rotations at this coord.
            if i == 0 {
                // There aren't any more possibilities.
                break;
            }
            records[x][y][z] = 0; // Reset tries.
            i -= 1; // Backtrack.
        }
        recipe_builder.remove(&coords[i]);
    }
    if successes != 0 {
        println!("Yes! {:?}", successes);
    }
    successes
}
