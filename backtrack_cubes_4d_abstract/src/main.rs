extern crate hoffman;

use hoffman::*;
use hoffman::four::*;
use std::time::Instant;
use std::collections::HashMap;

fn main() {
    /* Narrow: 8003461 Planes */
    let brick = [57, 59, 62, 63];
    /**/

    /* Wide: 8003461 Planes
    const BRICK: [IntType; N] = [53, 54, 57, 59];
    const SIDESUM: IntType = 53 + 54 + 57 + 59;
    */

    /* Balanced: 9929841 Planes
    const BRICK: [IntType; N] = [1, 2, 3, 4];
    const SIDESUM: IntType = 1 + 2 + 3 + 4;
    */
    println!("Brick: {:?}", brick);

    let comparator = Comparator::constructor(&brick);

    println!("Will determine kernels.");
    let now = Instant::now();
    let unique_kernels = generate_kernels(&brick);
    println!("Unique kernel count: {:?}", unique_kernels.len());
    println!("Time spent making kernels: {:?} s", now.elapsed().as_secs());

    println!("Will determine number of unique cubes...");
    let now = Instant::now();
    let mut unique_cube_count: usize = 0;
    for (i, kernel) in unique_kernels.iter().enumerate() {
        unique_cube_count += pack_cube(&brick, &kernel, &comparator);
        println!("Count: {:?}", unique_cube_count);
        if i % 50 == 0 {
            println!("{:.2}%", 100.0 * i as f64 / unique_kernels.len() as f64);
        }
    }
    println!("Total unique plane count: {:?}", unique_cube_count);
    println!("Time spent making planes: {:?} s", now.elapsed().as_secs());
}

fn generate_kernels(brick: &Brick) -> Vec<cube::Kernel> {

    let mut kernels: Vec<cube::Kernel> = Vec::new();

    let mut faces = [Point3D { x: 0, y: 0, z: 0 }; N * (N - 1) * (N - 2)];
    for (i, permutation) in permutations(brick, 3).iter().enumerate() {
        faces[i] = Point3D { x: permutation[0], y: permutation[1], z: permutation[2] };
    }
    println!("Kernel Faces: {:?} ({:?} in total)", faces, faces.len());

    let mut coords = [(0, 0, 0); KERNEL_DIM * KERNEL_DIM * KERNEL_DIM];
    for (i, coord) in coords.iter_mut().enumerate() {
        coord.0 = (i / (KERNEL_DIM * KERNEL_DIM)) % KERNEL_DIM;
        coord.1 = (i / KERNEL_DIM) % KERNEL_DIM;
        coord.2 = i % KERNEL_DIM;
    }
    println!("Kernel Coordinates: {:?}", coords);

    let mut sizes = [[[Point3D { x: 0, y: 0, z: 0 }; KERNEL_DIM]; KERNEL_DIM]; KERNEL_DIM];
    let mut records = [[[0; KERNEL_DIM]; KERNEL_DIM]; KERNEL_DIM];
    let max_tries = faces.len();

    let mut i: usize = 0;
    let mut iteration: usize = 0;
    let mut successes: usize = 0;
    loop {
        if i == KERNEL_DIM * KERNEL_DIM * KERNEL_DIM { // We have successfully placed bricks everywhere.
            //kernels.push(sizes);
            successes += 1;
            /*if kernels.len() >= 100000 {
                break;
            }*/
            i = i - 1; // Carry on.
        }

        let coord = coords[i];
        let (x, y, z) = coord;

        if records[x][y][z] == max_tries { // We have tried all faces at this coord.
            records[x][y][z] = 0; // Reset tries.
            sizes[x][y][z] = Point3D { x: 0, y: 0, z: 0 }; // Remove brick from sizes.
            if i == 0 {
                // There aren't any more possibilities.
                break;
            } else {
                i -= 1 // Backtrack.
            }
        } else { // We'll try placing a brick.
            sizes[x][y][z] = faces[records[x][y][z]]; // Fetch next rotation and place brick.
            if cube::kernel_is_brick_valid(&sizes, coord) {
                i += 1; // Go to next coord.
            }
            records[x][y][z] += 1; // Register that this rotation has been tried.
        }
        iteration += 1;
        if iteration % 100_000_000 == 0 {
            println!("Status at iteration {:?} is: {:?}, sucs: {:?}", iteration, records[0][0][0], successes);
        }
    }
    println!("Total kernels: {:?}", successes);

    println!("Kernel count including rotations and reflections: {:?}", kernels.len());
    //cube::kernel_drain_symmetries(&mut kernels);
    kernels
}

fn pack_cube(brick: &Brick, kernel: &cube::Kernel, comparator: &Comparator) -> usize {

    let mut rotations = [Point3D { x: 0, y: 0, z: 0 }; N * (N - 1) * (N - 2)];
    for (i, permutation) in permutations(brick, 3).iter().enumerate() {
        rotations[i] = Point3D { x: permutation[0], y: permutation[1], z: permutation[2] };
    }
    //println!("Rotations: {:?}", rotations.len());

    let mut coords = [(0, 0, 0); N * N * N];
    for (i, coord) in coords.iter_mut().enumerate() {
        coord.0 = (i / (N * N)) % N;
        coord.1 = (i / N) % N;
        coord.2 = i % N;
    }
    //println!("Coords: {:?}", coords.len());

    //let mut plane_sizes = Vec::new();

    let mut sizes = [[[Point3D { x: 0, y: 0, z: 0 }; N]; N]; N];
    let mut positions = [[[Point3D { x: 0, y: 0, z: 0 }; N]; N]; N];
    let mut records = [[[0; N]; N]; N];

    let mut i: usize = 0;
    let mut iteration: usize = 0;
    let mut successes: usize = 0;
    let mut type_counts: HashMap<IntType, usize> = HashMap::with_capacity(N);

    for x in 0..2 {
        for y in 0..2 {
            for z in 0..2 {
                let brick = kernel[x][y][z];
                sizes[if x == 0 { 0 } else { N - 1 }][if y == 0 { 0 } else { N - 1 }][if z == 0 { 0 } else { N - 1 }] = brick;
                let brick_sum = brick.x + brick.y + brick.z;
                let type_count = type_counts.entry(brick_sum).or_insert(0);
                *type_count += 1;
            }
        }
    }
    println!("sizes {:?}", sizes);

    fn is_inside_kernel(x: usize, y: usize, z: usize) -> bool {
        (0 == x || x == N - 1)
        && (0 == y || y == N - 1)
        && (0 == z || z == N - 1)
    }

    println!("Typecounts: {:?}", type_counts);

    loop {
        if i == N * N * N { // We have successfully placed N * N bricks.
            //plane_sizes.push(sizes);
            successes += 1;
            cube::plot(&positions, &sizes, brick, &format!("Cube at iteration {}", iteration));
            i = i - 1; // Carry on.
            let (px, py, pz) = coords[i];
            decrement_type_count(&mut type_counts, &sizes[px][py][pz]);
        }

        let coord = coords[i];
        let (x, y, z) = coord;
        let inside_kernel = is_inside_kernel(x, y, z);
        let max_tries = if inside_kernel { 1 } else { rotations.len() };

        if records[x][y][z] == max_tries { // We have tried all rotations at this coord.
            if i == 0 {
                // There aren't any more possibilities.
                //println!("We have tried everything! :)");
                break;
            }
            //print!("Remove form {:?}: {:?} ", next_brick_sum, type_count);

            //println!("Removed: {:?}", type_counts[&next_brick_sum]);
            //println!("Iteration {:?}, record: {:?}", iteration, records[0][0][0]);
            records[x][y][z] = 0; // Reset tries.
            if !inside_kernel {
                sizes[x][y][z] = Point3D { x: 0, y: 0, z: 0 }; // Remove brick from sizes.
            }
            positions[x][y][z] = Point3D { x: 0, y: 0, z: 0 }; // Remove brick from positions.
            i -= 1; // Backtrack.
            let (px, py, pz) = coords[i];
            if !is_inside_kernel(px, py, pz) {
                decrement_type_count(&mut type_counts, &sizes[px][py][pz]);
            }
            //println!("After: {:?} vs. {:?}", type_counts.values().fold(0, |sum, &v| sum + v), i);
        } else { // We'll try placing a brick.
            if !inside_kernel { // Fetch next rotation and place brick.
                let next_brick = rotations[records[x][y][z]];
                let next_brick_sum = next_brick.x + next_brick.y + next_brick.z;
                let type_count = type_counts.entry(next_brick_sum).or_insert(0);
                if *type_count < 16 {
                    sizes[x][y][z] = next_brick;
                    cube::position_brick(&mut positions, &sizes, coord);
                    if cube::is_brick_valid(&positions, &sizes, coord, &comparator) {
                        *type_count += 1;
                        i += 1; // Go to next coord.
                    }
                }
            } else {
                cube::position_brick(&mut positions, &sizes, coord);
                if cube::is_brick_valid(&positions, &sizes, coord, &comparator) {
                    i += 1; // Go to next coord.
                }
            }
            records[x][y][z] += 1; // Register that this rotation has been tried.
        }

        //assert!(type_counts.values().fold(0, |sum, &v| sum + v) == i, "Invalid type count.");

        iteration += 1;
        if iteration % 10_000_000 == 0 {
            println!("Status: Iteration {:?}, i: {:?}, successes: {:?}, record: {:?}", iteration, i, successes, records[0]);
            println!("Type counts: {:?}", type_counts);
        }
    }
    if successes != 0 {
        println!("Yes! {:?}", successes);
    }
    successes
    /*let before_count = plane_sizes.len();
    if plane::kernel_has_symmetries(kernel) {
        plane::drain_symmetries(&mut plane_sizes);
        println!("Symmetric kernel: Reduced planes from {:?} to {:?}", before_count, plane_sizes.len());
    }
    plane_sizes.len()*/
}

fn decrement_type_count(types: &mut HashMap<IntType, usize>, brick: &Point3D) {
    let brick_sum = brick.x + brick.y + brick.z;
    let count = types.entry(brick_sum).or_insert(0);
    *count -= 1;
}
