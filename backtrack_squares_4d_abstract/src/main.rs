extern crate hoffman;

use hoffman::*;
use hoffman::four::*;
use std::time::Instant;

fn main() {
    /* Narrow: 8003461 Squares */
    let brick = [57, 59, 62, 63];
    /**/

    /* Wide: 8003461 Squares
    const BRICK: [IntType; N] = [53, 54, 57, 59];
    const SIDESUM: IntType = 53 + 54 + 57 + 59;
    */

    /* Balanced: 9929841 Squares
    const BRICK: [IntType; N] = [1, 2, 3, 4];
    const SIDESUM: IntType = 1 + 2 + 3 + 4;
    */
    println!("Brick: {:?}", brick);

    let comparator = Comparator::constructor(&brick);

    println!("Will determine kernels.");
    let now = Instant::now();
    let unique_kernels = generate_two_by_two_kernels(&brick);
    println!("Unique kernel count: {:?}", unique_kernels.len());
    println!("Time spent making kernels: {:?} s", now.elapsed().as_secs());

    for (i, kernel) in unique_kernels.iter().filter(|k| square::kernel_is_self_symmetric(k)).enumerate() {
        let sub_group_index = square::kernel_symmetries(kernel).iter().filter(|&s| s == kernel).count();
        let name = format!("{} self symmetries, kernel {}", sub_group_index, i);
        square::kernel_plot(kernel, &brick, &name);
    }

    println!("Will determine number of unique squares...");
    let now = Instant::now();
    let mut unique_square_count: usize = 0;
    let mut total_iterations: usize = 0;
    for (i, kernel) in unique_kernels.iter().enumerate() {
        let (count, iterations) = pack_four_by_four(&brick, &kernel, &comparator);;
        unique_square_count += count;
        total_iterations += iterations;
        if i % 50 == 0 {
            println!("{:.2}%", 100.0 * i as f64 / unique_kernels.len() as f64);
        }
    }
    println!("Total unique square count: {:?}", unique_square_count);
    println!("Total iterations: {:?}", total_iterations);
    println!("Time spent making squares: {:?} s", now.elapsed().as_secs());
}

fn generate_two_by_two_kernels(brick: &Brick) -> Vec<square::Kernel> {

    let mut kernels: Vec<square::Kernel> = Vec::new();

    let mut faces = [Point2D { x: 0, y: 0 }; N * (N - 1)];
    for (i, permutation) in permutations(brick, 2).iter().enumerate() {
        faces[i] = Point2D { x: permutation[0], y: permutation[1] };
    }
    //println!("Kernel Faces: {:?}", faces);

    let mut coords = [(0, 0); KERNEL_DIM * KERNEL_DIM];
    for (i, coord) in coords.iter_mut().enumerate() {
        coord.0 = i / KERNEL_DIM;
        coord.1 = i % KERNEL_DIM;
    }
    //println!("Kernel Coordinates: {:?}", coords);

    let mut sizes = [[Point2D { x: 0, y: 0 }; KERNEL_DIM]; KERNEL_DIM];
    let mut records = [[0; KERNEL_DIM]; KERNEL_DIM];
    let max_tries = faces.len();

    let mut i: usize = 0;
    loop {
        if i == KERNEL_DIM * KERNEL_DIM { // We have successfully placed bricks everywhere.
            kernels.push(sizes);
            i = i - 1; // Carry on.
        }

        let coord = coords[i];
        let (x, y) = coord;

        if records[x][y] == max_tries { // We have tried all faces at this coord.
            records[x][y] = 0; // Reset tries.
            sizes[x][y] = Point2D { x: 0, y: 0 }; // Remove brick from sizes.
            if i == 0 {
                // There aren't any more possibilities.
                break;
            } else {
                i -= 1 // Backtrack.
            }
        } else { // We'll try placing a brick.
            sizes[x][y] = faces[records[x][y]]; // Fetch next rotation and place brick.
            if square::kernel_is_brick_valid(&sizes, coord) {
                i += 1; // Go to next coord.
            }
            records[x][y] += 1; // Register that this rotation has been tried.
        }
    }

    println!("Kernel count including rotations and reflections: {:?}", kernels.len());
    square::kernel_drain_symmetries(&mut kernels);
    kernels
}

fn pack_four_by_four(brick: &Brick, kernel: &square::Kernel, comparator: &Comparator) -> (usize, usize) {

    let mut rotations = [Point2D { x: 0, y: 0 }; N * (N - 1)];
    for (i, permutation) in permutations(brick, 2).iter().enumerate() {
        rotations[i] = Point2D { x: permutation[0], y: permutation[1] };
    }
    //println!("Rotations: {:?}", rotations);

    let mut coords = [(0, 0); N * N];
    for (i, coord) in coords.iter_mut().enumerate() {
        coord.0 = i / N;
        coord.1 = i % N;
    }
    //println!("Coordinates: {:?}", coords);

    let mut square_sizes = Vec::new();

    let mut sizes = [[Point2D { x: 0, y: 0 }; N]; N];
    let mut positions = [[Point2D { x: 0, y: 0 }; N]; N];
    let mut records = [[0; N]; N];

    let mut i: usize = 0;
    let mut iteration: usize = 0;
    //let mut successes: usize = 0;
    loop {
        if i == N * N { // We have successfully placed N * N bricks.
            square_sizes.push(sizes);
            //successes += 1;
            i = i - 1; // Carry on.
        }

        let coord = coords[i];
        let (x, y) = coord;
        let inside_kernel = 0 < x && x < N - 1 && 0 < y && y < N - 1;
        let max_tries = if inside_kernel { 1 } else { rotations.len() };

        if records[x][y] == max_tries { // We have tried all rotations at this coord.
            records[x][y] = 0; // Reset tries.
            sizes[x][y] = Point2D { x: 0, y: 0 }; // Remove brick from sizes.
            positions[x][y] = Point2D { x: 0, y: 0 }; // Remove brick from positions.
            if i == 0 {
                // There aren't any more possibilities.
                //println!("We have tried everything! :)");
                break;
            } else {
                i -= 1 // Backtrack.
            }
        } else { // We'll try placing a brick.
            sizes[x][y] = if inside_kernel { // Fetch next rotation and place brick.
                kernel[x - 1][y - 1]
            } else {
                rotations[records[x][y]]
            };
            square::position_brick(&mut positions, &sizes, coord);
            if square::is_brick_valid(&positions, &sizes, coord, &comparator) {
                i += 1; // Go to next coord.
            }
            records[x][y] += 1; // Register that this rotation has been tried.
        }

        iteration += 1;
        if iteration % 10_000_000 == 0 {
            println!("Status at iteration {:?} is: {:?}", iteration, records[0][0]);
        }
    }
    let before_count = square_sizes.len();
    if square::kernel_is_self_symmetric(kernel) {
        square::drain_symmetries(&mut square_sizes);
        println!("Symmetric kernel: Reduced squares from {:?} to {:?}", before_count, square_sizes.len());
    }
    (square_sizes.len(), iteration)
}
