extern crate hoffman;

use hoffman::*;
use hoffman::three::*;
use std::time::Instant;
//use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let brick: Brick = [4, 5, 6];
    println!("Brick: {:?}", brick);

    let comparator = Comparator::constructor(&brick);

    println!("Will determine packings.");
    let now = Instant::now();
    let mut packings = generate_cubes(&brick, &comparator);
    println!("Total packings count: {:?}", packings.len());
    cube::drain_symmetries(&mut packings);
    println!("Total unique packings count: {:?}", packings.len());

    for (i, &(positions, sizes)) in packings.iter().enumerate() {
        let name = format!("3D Packing {}", i);
        cube::plot(&positions, &sizes, &brick, &name);
    }

    check_duality(packings, &brick, &comparator);

    println!("Time spent making packings: {:?} s", now.elapsed().as_secs());

}

fn generate_cubes(brick: &Brick, comparator: &Comparator) -> Vec<(cube::Cube, cube::Cube)> {
    let mut type_counts: Vec<Vec<HashMap<IntType, usize>>> = Vec::new();
    for _dim in 0..N {
        let mut dim_counts = Vec::new();
        for _level in 0..N {
            dim_counts.push(HashMap::with_capacity(N));
        }
        type_counts.push(dim_counts);
    }
    println!("{:?}", type_counts);

    let mut rotations = [Point3D { x: 0, y: 0, z: 0 }; N * (N - 1) * (N - 2)];
    for (i, permutation) in permutations(brick, 3).iter().enumerate() {
        rotations[i] = Point3D { x: permutation[0], y: permutation[1], z: permutation[2] };
    }
    println!("Rotations: {:?}", rotations.len());

    let mut coords = [(0, 0, 0); N * N * N];
    for (i, coord) in coords.iter_mut().enumerate() {
        coord.0 = (i / (N * N)) % N;
        coord.1 = (i / N) % N;
        coord.2 = i % N;
    }
    println!("Coords: {:?}", coords.len());

    let mut packings = Vec::new();

    let mut positions = [[[Point3D { x: 0, y: 0, z: 0 }; N]; N]; N];
    let mut sizes = [[[Point3D { x: 0, y: 0, z: 0 }; N]; N]; N];
    let mut records = [[[0; N]; N]; N];

    let mut i: usize = 0;
    let mut iteration: usize = 0;
    let mut successes: usize = 0;
    //let mut type_counts: HashMap<IntType, usize> = HashMap::with_capacity(N);

    loop {
        if i == N * N * N { // We have successfully placed all bricks.
            packings.push((positions, sizes));
            if successes == 0 {
                println!("Iterations: {:?}", iteration);
                println!("Records: {:?}", records);
            }
            successes += 1;
            i = i - 1; // Carry on.
            let (px, py, pz) = coords[i];
            decrement_type_count(&mut type_counts, &sizes[px][py][pz], [px, py, pz]);
            sizes[px][py][pz] = Point3D { x: 0, y: 0, z: 0 }; // Remove brick from sizes.
            positions[px][py][pz] = Point3D { x: 0, y: 0, z: 0 }; // Remove brick from positions.
        }

        let coord = coords[i];
        let (x, y, z) = coord;
        let max_tries = rotations.len();

        if records[x][y][z] == max_tries { // We have tried all rotations at this coord.
            if i == 0 {
                // There aren't any more possibilities.
                println!("We have tried everything! :)");
                break;
            }
            records[x][y][z] = 0; // Reset tries.
            i -= 1; // Backtrack.
            let (px, py, pz) = coords[i];
            decrement_type_count(&mut type_counts, &sizes[px][py][pz], [px, py, pz]);
            sizes[px][py][pz] = Point3D { x: 0, y: 0, z: 0 }; // Remove brick from sizes.
            positions[px][py][pz] = Point3D { x: 0, y: 0, z: 0 }; // Remove brick from positions.
        } else { // We'll try placing a brick.
            let next_brick = rotations[records[x][y][z]];
            records[x][y][z] += 1; // Register that this rotation has been tried.
            sizes[x][y][z] = next_brick;
            cube::position_brick(&mut positions, &sizes, coord);
            increment_type_count(&mut type_counts, &next_brick, [x, y, z]);
            if cube::is_brick_valid(&positions, &sizes, coord, &comparator)
            && validate_type_count(&type_counts, &next_brick, [x, y, z])
            && !cube::makes_sharp_corner(&positions, &sizes, [x, y, z], &comparator) {
                i += 1; // Go to next coord.
            } else {
                sizes[x][y][z] = Point3D { x: 0, y: 0, z: 0 }; // Remove brick from sizes.
                positions[x][y][z] = Point3D { x: 0, y: 0, z: 0 }; // Remove brick from positions.
                decrement_type_count(&mut type_counts, &next_brick, [x, y, z]);
            }
        }

        iteration += 1;
        if iteration % 100_000 == 0 {
            println!("Status: Iteration {:?}, i: {:?}, successes: {:?}, record: {:?}", iteration, i, successes, records[0][0]);
        }
    }
    println!("Total iterations {:?}", iteration);
    packings
}

fn validate_type_count(counts: &Vec<Vec<HashMap<IntType, usize>>>, brick: &Point3D, coord: [usize; N]) -> bool {
    coord.iter().enumerate().all(|(i, &v)| counts[i][v][&brick[i]] <= N)
}

fn decrement_type_count(counts: &mut Vec<Vec<HashMap<IntType, usize>>>, brick: &Point3D, coord: [usize; N]) {
    for i in 0..N {
        let count = counts[i][coord[i]].entry(brick[i]).or_insert(0);
        *count -= 1;
    }
}

fn increment_type_count(counts: &mut Vec<Vec<HashMap<IntType, usize>>>, brick: &Point3D, coord: [usize; N]) {
    for i in 0..N {
        let count = counts[i][coord[i]].entry(brick[i]).or_insert(0);
        *count += 1;
    }
}

fn check_duality(packings: Vec<(cube::Cube, cube::Cube)>, brick: &Brick, comparator: &Comparator) {
    let dims = (0..N).collect::<Vec<_>>();
    let permutations = permutations(&dims, N);
    for permutation in permutations.iter() {
        println!("Checking for dual using permutation {:?}:", permutation);
        let res = packings.iter().enumerate().map(|(i, &(_positions, sizes))| {
            if let Some(_) = apply_permutation(&sizes, &permutation, brick, comparator) {
                format!("{}", i)
            } else {
                format!("")
            }
        }).filter(|s| s.len() > 0 ).collect::<Vec<_>>().join(", ");
        println!("{}", res);
    }
}

fn apply_permutation(sizes: &cube::Cube, permutation: &[usize], brick: &Brick, comparator: &Comparator) -> Option<(cube::Cube, cube::Cube)> {
    let mut map = HashMap::new();
    for (i, v) in brick.iter().enumerate() {
        map.insert(v, i);
    }
    let mut perm_sizes = [[[Point3D::ZERO; N]; N]; N];
    let mut perm_positions = [[[Point3D::ZERO; N]; N]; N];
    let mut coords = [(0, 0, 0); N * N * N];
    for (i, coord) in coords.iter_mut().enumerate() {
        coord.0 = (i / (N * N)) % N;
        coord.1 = (i / N) % N;
        coord.2 = i % N;
    }
    for &(x, y, z) in coords.iter() {
        let size = sizes[x][y][z];
        let mut perm_size = Point3D::ZERO;
        for i in 0..N {
            perm_size[i] = brick[permutation[map[&size[i]]]];
        }
        perm_sizes[x][y][z] = perm_size;
        cube::position_brick(&mut perm_positions, &perm_sizes, (x, y, z));
        if !cube::is_brick_valid(&perm_positions, &perm_sizes, (x, y, z), comparator) {
            return None;
        }
    }
    let name = format!("3D Dual {:?}", permutation);
    cube::plot(&perm_positions, &perm_sizes, &brick, &name);
    Some((perm_positions, perm_sizes))
}
