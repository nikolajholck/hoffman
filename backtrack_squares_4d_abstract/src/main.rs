extern crate hoffman;

use hoffman::*;
use hoffman::four::*;
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    /* Narrow:    7807933 Squares
       Wide:      7807933 Squares
       Universal: 6406310 Squares */

    let bricks = [
        [57, 59, 62, 63], // Narrow
        [53, 54, 57, 59]  // Wide
    ];

    println!("Bricks: {:?}", bricks);

    assert!(bricks.iter().all(|&brick| list_has_unique_sums(&brick)), "Brick doesn't have unique sums.");

    println!("Will determine kernels.");
    let now = Instant::now();
    let unique_kernels = backtrack_kernels(&bricks[0]);
    println!("Unique kernel count: {:?}", unique_kernels.len());
    println!("Time spent making kernels: {:?} s", now.elapsed().as_secs());

    for (i, kernel) in unique_kernels.iter().filter(|k| square::kernel_is_self_symmetric(k)).enumerate() {
        let sub_group_index = square::kernel_symmetries(kernel).iter().filter(|&s| s == kernel).count();
        let name = format!("{} self symmetries, kernel {}", sub_group_index, i);
        square::kernel_plot(kernel, &bricks[0], &name);
    }

    println!("Will determine number of unique squares...");
    let now = Instant::now();
    let mut unique_square_count: usize = 0;
    let mut total_iterations: usize = 0;
    for (i, kernel) in unique_kernels.iter().enumerate() {
        let (count, iterations) = backtrack_squares(&bricks, &kernel);
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

fn backtrack_kernels(brick: &Brick) -> Vec<square::Kernel> {

    let mut kernels = Vec::new();

    let mut rotations = [Point2D { x: 0, y: 0 }; N * (N - 1)];
    for (i, permutation) in permutations(brick, square::M).iter().enumerate() {
        rotations[i] = Point2D { x: permutation[0], y: permutation[1] };
    }

    let coords = square::make_coords([KERNEL_DIM; square::M]);
    //println!("Kernel coordinates: {:?}", coords);

    let mut sizes = [[Point2D { x: 0, y: 0 }; KERNEL_DIM]; KERNEL_DIM];
    let mut records = [[0; KERNEL_DIM]; KERNEL_DIM];

    let max_tries = rotations.len();

    let mut i: usize = 0;

    loop {
        let coord = coords[i];
        let (x, y) = (coord[0], coord[1]);

        if records[x][y] < max_tries { // We'll try placing a brick.
            sizes[x][y] = rotations[records[x][y]]; // Fetch next rotation and place brick.
            records[x][y] += 1; // Register that this rotation has been tried.
            if square::kernel_is_brick_valid(&sizes, &coord) {
                if i == KERNEL_DIM * KERNEL_DIM - 1 { // We have successfully placed bricks everywhere.
                    kernels.push(sizes);
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
        let coord = coords[i];
        let (x, y) = (coord[0], coord[1]);
        sizes[x][y] = Point2D::ZERO; // Remove brick from sizes.
    }

    println!("Kernel count including rotations and reflections: {:?}", kernels.len());
    square::kernel_drain_symmetries(&mut kernels);
    kernels
}

struct Packing {
    positions: square::Square,
    sizes: square::Square,
    bricks: Vec<Point2D>
}

impl Packing {
    fn new(brick: &Brick) -> Packing {
        Packing {
            positions: [[Point2D::ZERO; N]; N],
            sizes: [[Point2D::ZERO; N]; N],
            bricks: permutations(brick, square::M).iter().map(|permutation| {
                Point2D { x: permutation[0], y: permutation[1] }
            }).collect()
        }
    }

    fn place(&mut self, coord: &square::Coord, brick_index: usize) {
        let (x, y) = (coord[0], coord[1]);
        self.sizes[x][y] = self.bricks[brick_index];
        square::position_brick(&mut self.positions, &self.sizes, &coord);
    }

    fn remove(&mut self, coord: &square::Coord) {
        let (x, y) = (coord[0], coord[1]);
        self.sizes[x][y] = Point2D::ZERO; // Remove brick from sizes.
        self.positions[x][y] = Point2D::ZERO; // Remove brick from positions.
    }

    fn is_valid(&self, coord: &square::Coord) -> bool {
        square::is_brick_valid(&self.positions, &self.sizes, &coord)
        //&& !square::makes_sharp_corner(&self.positions, &self.sizes, &coord)
    }
}

fn backtrack_squares(bricks: &[Brick], kernel: &square::Kernel) -> (usize, usize) {
    let mut packings: Vec<Packing> = bricks.iter().map(|brick| Packing::new(brick)).collect();

    let coords = square::make_coords([N; square::M]);
    //println!("Coordinates: {:?}", coords);

    let mut kernel_map: HashMap<(usize, usize), usize> = HashMap::new();
    for x in 0..KERNEL_DIM {
        for y in 0..KERNEL_DIM {
            let kernel_brick = kernel[x][y];
            let brick_index = permutations(&bricks[0], square::M).iter().position(|permutation| {
                permutation[0] == kernel_brick[0] && permutation[1] == kernel_brick[1]
            }).unwrap();
            kernel_map.insert((x + 1, y + 1), brick_index);
        }
    }

    let mut squares = Vec::new();

    let mut records = [[0; N]; N];

    let mut i: usize = 0;
    let mut iteration: usize = 0;

    loop {
        iteration += 1;

        let coord = coords[i];
        let (x, y) = (coord[0], coord[1]);
        let inside_kernel = kernel_map.contains_key(&(x, y));
        let max_tries = if inside_kernel { 1 } else { N * (N - 1) };

        if records[x][y] < max_tries { // We'll try placing a brick.
            let brick_index = if inside_kernel { // Fetch next rotation and place brick.
                kernel_map[&(x, y)]
            } else {
                records[x][y]
            };
            for packing in &mut packings {
                packing.place(&coord, brick_index);
            }
            records[x][y] += 1; // Register that this rotation has been tried.

            if packings.iter().all(|packing| packing.is_valid(&coord)) {
                if i == N * N - 1 { // We have successfully placed all bricks.
                    squares.push(packings[0].sizes);
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
        for packing in &mut packings {
            packing.remove(&coords[i]);
        }
    }
    let before_count = squares.len();
    if square::kernel_is_self_symmetric(kernel) {
        square::drain_symmetries(&mut squares);
        println!("Symmetric kernel: Reduced squares from {:?} to {:?}", before_count, squares.len());
    }
    (squares.len(), iteration)
}
