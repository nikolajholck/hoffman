extern crate hoffman;

use std::time::Instant;
use hoffman::*;

const N: usize = 3;

fn main() {
    let dimension_tuples = vec!(
        vec!(4, 5, 6),
        //vec!(6, 5, 4)
    );
    println!("Dimension Tuples: {:?}", dimension_tuples);

    assert!(dimension_tuples.iter().all(|dimension_tuple| utils::list_has_unique_sums(dimension_tuple)), "Dimension tuple does not have unique sums.");

    println!("Will determine recipes.");
    let now = Instant::now();
    let recipes = backtrack_cubes(&dimension_tuples);
    println!("Total recipes count: {:?}", recipes.len());
    let recipes = Recipe::find_unique(recipes);
    println!("Total unique recipes count: {:?}", recipes.len());

    let name = format!("3d-universal-recipes");
    plot_multiple(&recipes, &dimension_tuples[0], &name);

    for (i, recipe) in recipes.iter().enumerate() {
        let name = format!("3D Packing {}", i);
        plot::plot_3d(recipe, &dimension_tuples[0], &name);
        recipe.save_json(&String::from("cubes"), &name);
    }

    compute_distances(&recipes);
    check_duality(&recipes, &dimension_tuples);

    println!("Time spent making recipes: {:?}", now.elapsed());
}

fn backtrack_cubes(dimension_tuples: &Vec<DimensionTuple>) -> Vec<Recipe> {
    let mut recipe_builder = RecipeBuilder::new(N, N, dimension_tuples.clone());

    let coords = recipe_builder.get_recipe().map.coords().clone();
    println!("Coords: {:?}", coords.len());
    let mut recipes = Vec::new();

    let mut records = [[[0; N]; N]; N];
    let mut i: usize = 0;
    let mut iteration: usize = 0;
    let mut successes: usize = 0;

    let max_tries = N * (N - 1) * (N - 2);

    let indices: Vec<usize> = (0..N).collect();
    let perms = combinatorics::permutations(&indices.as_slice(), N);

    loop {
        iteration += 1;
        if iteration % 100_000 == 0 {
            println!("Iteration {:?}, i: {:?}, successes: {:?}", iteration, i, successes);
        }

        let coord = &coords[i];
        let (x, y, z) = (coord[0], coord[1], coord[2]);

        if records[x][y][z] < max_tries { // We'll try placing a brick.
            recipe_builder.insert(coord, &perms[records[x][y][z]]);
            records[x][y][z] += 1; // Register that this rotation has been tried.

            if recipe_builder.is_valid(coord) {
                if i == N * N * N - 1 { // We have successfully placed all bricks.
                    recipes.push(recipe_builder.get_recipe().clone());
                    if successes == 0 {
                        println!("Iterations: {:?}", iteration);
                        println!("Records: {:?}", records);
                    }
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
            records[x][y][z] = 0; // Reset tries.
            i -= 1; // Backtrack.
        }
        recipe_builder.remove(&coords[i]);
    }
    println!("Total iterations {:?}", iteration);
    recipes
}

fn compute_distances(recipes: &Vec<Recipe>) {
    for (i, a) in recipes.iter().enumerate() {
        print!("Packing {:2}: ", i);
        let all_distances: Vec<_> = recipes.iter().map(|b| a.distance_to(b) ).collect();
        let mut other_distances = all_distances.clone();
        other_distances.remove(i);
        let closest = other_distances.iter().min().unwrap();
        let farthest = other_distances.iter().max().unwrap();
        print!("Closest: {:2}, Farthest: {:2} ", closest, farthest);
        for (k, d) in all_distances.iter().enumerate().filter(|(_, d)| *d == closest) {
            print!("({:2}, {:2}) ", k, d);
        }
        println!();
    }
}

fn check_duality(recipes: &Vec<Recipe>, dimension_tuples: &Vec<DimensionTuple>) {
    let permutations = vec!(vec!(2, 1, 0));
    //let permutations = combinatorics::permutations(&(0..3).collect::<Vec<_>>(), 3);
    for permutation in &permutations {
        println!("Checking for dual using permutation {:?}:", permutation);
        let res = recipes.iter().enumerate().map(|(i, recipe)| {
            if check_permutation(recipe, permutation, dimension_tuples) {
                format!("{:?}", i)
            } else {
                format!("")
            }
        }).filter(|s| s.len() > 0 ).collect::<Vec<_>>().join(", ");
        println!("{}", res);
    }
}

fn check_permutation(recipe: &Recipe, permutation: &Orientation, dimension_tuples: &Vec<DimensionTuple>) -> bool {
    let recipe_builder = RecipeBuilder::generate(&recipe.pre_permute(permutation), dimension_tuples.clone());
    recipe_builder.validate()
}

pub fn plot_multiple(recipes: &Vec<Recipe>, dimension_tuple: &DimensionTuple, name: &String) {
    let mut squares = Vec::new();
    for (q, recipe) in recipes.iter().enumerate() {
        let recipe_builder = RecipeBuilder::generate(recipe, vec!(dimension_tuple.clone()));
        for dim in 0..1 {
            for level in 0..N {
                let rects = recipe_builder.get_rectangles_at(vec!((dim, level)));
                let plot_name = format!("{}", q + 1);
                let plot = plot::Plot {
                    name: if level == 2 { Some(plot_name) } else { None },
                    rectangles: rects
                };
                squares.push(plot);
            }
        }
    }
    let plots: Vec<plot::Plot> = (0..recipes.len() * N).map(|i| {
        let row = i / (7 * 3);
        let column = i % 7;
        let level = (i % (7 * 3)) / 7;
        squares[(row * 7 + column) * 3 + level].clone()
    }).collect();

    let figure = plot::Figure {
        name: None,
        plots: plots,
        dimension_tuple: dimension_tuple.to_vec(),
        rows: N * N,
        columns: recipes.len() / N
    };
    figure.save_svg(&String::from("cubes"), name);
    figure.save_tikz(&String::from("cubes"), name);
}
