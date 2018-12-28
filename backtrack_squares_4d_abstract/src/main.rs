extern crate hoffman;

use hoffman::*;
use std::time::Instant;

const N: usize = 4;
const M: usize = 2;

fn main() {
    /* Unique:
       Equal:     9 680 536 squares
       Narrow:    7 807 933 squares
       Wide:      7 807 933 squares
       Universal: 6 406 310 squares */

   /* Including symmetries:
      Equal:     77 436 138 squares (Spiridonov)
      Narrow:    62 458 582 squares
      Wide:      62 458 582 squares
      Universal: 51 247 458 squares */

    let dimension_tuples = vec!(
        vec!(8, 9, 10, 12),  // Wide
        vec!(10, 12, 13, 14) // Narrow
    );

    println!("Dimension tuples: {:?}", dimension_tuples);

    assert!(dimension_tuples.iter().all(|dimension_tuple| utils::list_has_unique_sums(&dimension_tuple)), "Brick does not have unique sums.");

    println!("Will determine kernels.");
    let now = Instant::now();
    let unique_kernels = backtrack_kernels(&dimension_tuples);
    println!("Unique kernel count: {:?}", unique_kernels.len());
    println!("Time spent making kernels: {:?} s", now.elapsed().as_secs());

    for (i, kernel) in unique_kernels.iter().filter(|k| k.is_self_symmetric()).enumerate() {
        let sub_group_index = kernel.symmetries().iter().filter(|&s| s == kernel).count();
        let name = format!("{} self symmetries, kernel {}", sub_group_index, i);
        kernel_plot(kernel, &dimension_tuples[0], &name);
    }

    println!("Will determine number of unique squares...");
    let now = Instant::now();
    let (unique_square_count, total_iterations) = unique_kernels.iter().map(|kernel| {
        backtrack_squares(&dimension_tuples, &kernel)
    }).fold((0, 0), |acc, v| (acc.0 + v.0, acc.1 + v.1));
    println!("Total unique square count: {:?}", unique_square_count);
    println!("Total iterations: {:?}", total_iterations);
    println!("Time spent making squares: {:?} s", now.elapsed().as_secs());
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
    let mut records = [[0; N]; N];

    let max_tries = perms.len();

    let mut i: usize = 0;

    loop {
        let coord = &coords[i];
        let (x, y) = (coord[0], coord[1]);

        if records[x][y] < max_tries { // We'll try placing a brick.
            recipe_builder.insert(coord, &perms[records[x][y]]); // Fetch next rotation and place brick.
            records[x][y] += 1; // Register that this rotation has been tried.
            if recipe_builder.satisfies_line_criterion(coord) {
                if i == M * M - 1 { // We have successfully placed bricks everywhere.
                    kernels.push(recipe_builder.get_recipe().clone());
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
            records[x][y] = 0; // Reset tries.
            i -= 1 // Backtrack.
        }
        recipe_builder.remove(&coords[i]);
    }

    println!("Kernel count including rotations and reflections: {:?}", kernels.len());
    Recipe::find_unique(kernels)
}

fn backtrack_squares(dimension_tuples: &Vec<DimensionTuple>, kernel: &Recipe) -> (usize, usize) {
    let mut recipe_builder = RecipeBuilder::new(N, M, dimension_tuples.clone());

    let coords = utils::make_coords(N, M);
    //println!("Coordinates: {:?}", coords);

    let indices: Vec<usize> = (0..N).collect();
    let perms: Vec<Orientation> = combinatorics::permutations(&indices.as_slice(), M);

    let mut squares = Vec::new();

    let mut records = [[0; N]; N];

    let mut i: usize = 0;
    let mut iteration: usize = 0;

    loop {
        iteration += 1;

        let coord = &coords[i];
        let (x, y) = (coord[0], coord[1]);
        let inside_kernel = kernel.map.contains_key(coord);
        let max_tries = if inside_kernel { 1 } else { N * (N - 1) };

        if records[x][y] < max_tries { // We'll try placing a brick.
            let next_perm = if inside_kernel { // Fetch next rotation and place brick.
                &kernel.map.get(coord).unwrap()
            } else {
                &perms[records[x][y]]
            };

            recipe_builder.insert(coord, &next_perm);

            records[x][y] += 1; // Register that this rotation has been tried.

            if recipe_builder.is_valid(coord) {
                if i == N * N - 1 { // We have successfully placed all bricks.
                    squares.push(recipe_builder.get_recipe().clone());
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
            records[x][y] = 0; // Reset tries.
            i -= 1 // Backtrack.
        }
        recipe_builder.remove(&coords[i]);
    }
    let before_count = squares.len();
    let unique_squares = if kernel.is_self_symmetric() {
        let unique = Recipe::find_unique(squares);
        println!("Symmetric kernel: Reduced squares from {:?} to {:?}", before_count, unique.len());
        unique
    } else {
        squares
    };
    (unique_squares.len(), iteration)
}

pub fn kernel_plot(kernel: &Recipe, dimension_tuple: &DimensionTuple, name: &String) {
    let mut plots = Vec::new();
    let center: IntType = dimension_tuple.iter().sum::<IntType>() / 2;

    for kernel in &kernel.symmetries() {
        let mut rects = Vec::new();
        for x in 1..=2 {
            for y in 1..=2 {
                let orientations = kernel.map.get(&vec!(x, y)).unwrap();
                let size: DimensionTuple = orientations.iter().map(|&i| dimension_tuple[i]).collect();
                let rectangle = plot::Rectangle {
                    x: center + if x == 1 { -size[0] } else { 0 },
                    y: center + if y == 1 { -size[1] } else { 0 },
                    width: size[0],
                    height: size[1]
                };
                rects.push(rectangle);
            }
        }
        let plot = plot::Plot {
            name: None,
            rectangles: rects
        };
        plots.push(plot);
    }
    let figure = plot::Figure {
        name: None,
        plots: plots,
        dimension_tuple: dimension_tuple.to_vec(),
        rows: 2,
        columns: 4
    };
    figure.save_svg(&String::from("squares/kernels"), name);
    //figure.save_tikz(&String::from("squares/kernels"), name);
}
