extern crate hoffman;

use hoffman::*;
use hoffman::four::*;
use std::time::Instant;
use std::collections::HashMap;

fn main() {
    let bricks = [
        [8, 9, 10, 12],  // Wide
        [10, 12, 13, 14] // Narrow
    ];

    println!("Bricks: {:?}", bricks);

    assert!(bricks.iter().all(|&brick| list_has_unique_sums(&brick)), "Brick doesn't have unique sums.");

    println!("Will determine kernels.");
    let now = Instant::now();
    let unique_kernels = backtrack_kernels(&bricks[0]);
    println!("Unique kernel count: {:?}", unique_kernels.len());
    println!("Time spent making kernels: {:?} s", now.elapsed().as_secs());

    println!("Will determine number of unique cubes...");
    let now = Instant::now();
    let mut unique_cube_count: usize = 0;
    for (i, kernel) in unique_kernels.iter().enumerate() {
        unique_cube_count += backtrack_cubes(&bricks, &kernel);
        println!("Count for kernel {:?}: {:?}", i, unique_cube_count);
    }
    println!("Total unique cube count: {:?}", unique_cube_count);
    println!("Time spent making cubes: {:?} s", now.elapsed().as_secs());
}

fn backtrack_kernels(brick: &Brick) -> Vec<cube::Kernel> {

    let mut kernels = Vec::new();

    let mut rotations = [Point3D { x: 0, y: 0, z: 0 }; N * (N - 1) * (N - 2)];
    for (i, permutation) in permutations(brick, cube::M).iter().enumerate() {
        rotations[i] = Point3D { x: permutation[0], y: permutation[1], z: permutation[2] };
    }
    println!("Kernel rotations: {:?} ({:?} in total)", rotations, rotations.len());

    let coords = cube::make_coords([KERNEL_DIM; cube::M]);
    println!("Kernel coordinates: {:?}", coords);

    let mut sizes = [[[Point3D { x: 0, y: 0, z: 0 }; KERNEL_DIM]; KERNEL_DIM]; KERNEL_DIM];
    let mut records = [[[0; KERNEL_DIM]; KERNEL_DIM]; KERNEL_DIM];
    let max_tries = rotations.len();

    let mut i: usize = 0;
    let mut iteration: usize = 0;
    let mut successes: usize = 0;

    loop {
        iteration += 1;
        if iteration % 100_000_000 == 0 {
            println!("Iteration {:?}, records: {:?}, successes: {:?}", iteration, records[0][0][0], successes);
        }

        let coord = coords[i];
        let (x, y, z) = (coord[0], coord[1], coord[2]);

        if records[x][y][z] < max_tries { // We'll try placing a brick.
            sizes[x][y][z] = rotations[records[x][y][z]]; // Fetch next rotation and place brick.
            records[x][y][z] += 1; // Register that this rotation has been tried.
            if cube::kernel_is_brick_valid(&sizes, &coord) {
                if i == KERNEL_DIM * KERNEL_DIM * KERNEL_DIM - 1 { // We have successfully placed bricks everywhere.
                    successes += 1;
                    kernels.push(sizes);
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
        let coord = coords[i];
        let (x, y, z) = (coord[0], coord[1], coord[2]);
        sizes[x][y][z] = Point3D::ZERO; // Remove brick from sizes.
    }
    println!("Total kernels: {:?}", successes);
    println!("Kernel count including rotations and reflections: {:?}", kernels.len());
    kernels
}

struct Packing {
    positions: cube::Cube,
    sizes: cube::Cube,
    bricks: Vec<Point3D>,
    type_counts: HashMap<IntType, usize>,
}

impl Packing {
    fn new(brick: &Brick) -> Packing {
        Packing {
            positions: [[[Point3D::ZERO; N]; N]; N],
            sizes: [[[Point3D::ZERO; N]; N]; N],
            bricks: permutations(brick, cube::M).iter().map(|permutation| {
                Point3D { x: permutation[0], y: permutation[1], z: permutation[2] }
            }).collect(),
            type_counts: HashMap::with_capacity(N)
        }
    }

    fn place(&mut self, coord: &cube::Coord, brick_index: usize) {
        let (x, y, z) = (coord[0], coord[1], coord[2]);
        let brick = self.bricks[brick_index];
        self.increment_type_count(&brick, &coord);
        self.sizes[x][y][z] = brick;
        cube::position_brick(&mut self.positions, &self.sizes, &coord);
    }

    fn remove(&mut self, coord: &cube::Coord) {
        let (x, y, z) = (coord[0], coord[1], coord[2]);
        let brick = self.sizes[x][y][z];
        self.decrement_type_count(&brick, &coord);
        self.sizes[x][y][z] = Point3D::ZERO; // Remove brick from sizes.
        self.positions[x][y][z] = Point3D::ZERO; // Remove brick from positions.
    }

    fn is_valid(&self, coord: &cube::Coord) -> bool {
        cube::is_brick_valid(&self.positions, &self.sizes, &coord)
        && self.validate_type_count(&coord)
        //&& !cube::makes_sharp_corner(&self.positions, &self.sizes, &coord)
    }

    fn validate_type_count(&self, _coord: &cube::Coord) -> bool {
        self.type_counts.values().max().unwrap() <= &(N * N)
    }

    fn decrement_type_count(&mut self, brick: &Point3D, _coord: &cube::Coord) {
        let brick_sum = brick.x + brick.y + brick.z;
        let count = self.type_counts.entry(brick_sum).or_insert(0);
        *count -= 1;
    }

    fn increment_type_count(&mut self, brick: &Point3D, _coord: &cube::Coord) {
        let brick_sum = brick.x + brick.y + brick.z;
        let count = self.type_counts.entry(brick_sum).or_insert(0);
        *count += 1;
    }
}

fn backtrack_cubes(bricks: &[Brick], kernel: &cube::Kernel) -> usize {
    let mut packings: Vec<Packing> = bricks.iter().map(|brick| Packing::new(brick)).collect();

    let coords = cube::make_coords([N; cube::M]);
    //println!("Coords: {:?}", coords.len());

    let mut kernel_map: HashMap<(usize, usize, usize), usize> = HashMap::new();
    for x in 0..KERNEL_DIM {
        for y in 0..KERNEL_DIM {
            for z in 0..KERNEL_DIM {
                let kernel_brick = kernel[x][y][z];
                let brick_index = permutations(&bricks[0], cube::M).iter().position(|permutation| {
                    permutation[0] == kernel_brick[0] && permutation[1] == kernel_brick[1] && permutation[2] == kernel_brick[2]
                }).unwrap();
                kernel_map.insert((x + 1, y + 1, z + 1), brick_index);
            }
        }
    }

    let mut cubes = Vec::new();

    let mut records = [[[0; N]; N]; N];

    let mut i: usize = 0;
    let mut iteration: usize = 0;
    let mut successes: usize = 0;

    loop {
        iteration += 1;
        if iteration % 10_000_000 == 0 {
            println!("Status: Iteration {:?}, i: {:?}, successes: {:?}, record: {:?}", iteration, i, successes, records[0]);
            println!("Type counts: {:?}", packings[0].type_counts);
        }

        let coord = coords[i];
        let (x, y, z) = (coord[0], coord[1], coord[2]);
        let inside_kernel = kernel_map.contains_key(&(x, y, z));
        let max_tries = if inside_kernel { 1 } else { N * (N - 1) * (N - 2) };

        if records[x][y][z] < max_tries { // We'll try placing a brick.
            let brick_index = if inside_kernel { // Fetch next rotation and place brick.
                kernel_map[&(x, y, z)]
            } else {
                records[x][y][z]
            };
            for packing in &mut packings {
                packing.place(&coord, brick_index);
            }
            records[x][y][z] += 1; // Register that this rotation has been tried.

            if packings.iter().all(|packing| packing.is_valid(&coord)) {
                if i == N * N * N - 1 { // We have successfully placed all bricks.
                    successes += 1;
                    cubes.push(packings[0].sizes);
                    cube::plot(&packings[0].positions, &packings[0].sizes, &bricks[0], &format!("Cube at iteration {}", iteration));
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
        for packing in &mut packings {
            packing.remove(&coords[i]);
        }
    }
    if successes != 0 {
        println!("Yes! {:?}", successes);
    }
    successes
}
