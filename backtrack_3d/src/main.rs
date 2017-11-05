extern crate hoffman;

use hoffman::*;
use hoffman::three::*;
use std::time::Instant;
use std::collections::HashMap;
use std::iter::repeat;

fn main() {
    let bricks = [
        [4, 5, 6]
    ];
    println!("Bricks: {:?}", bricks);

    assert!(bricks.iter().all(|&brick| list_has_unique_sums(&brick)), "Brick doesn't have unique sums.");

    println!("Will determine packings.");
    let now = Instant::now();
    let mut packings = backtrack_cubes(&bricks);
    println!("Total packings count: {:?}", packings.len());
    cube::drain_symmetries(&mut packings);
    println!("Total unique packings count: {:?}", packings.len());

    for (i, &(positions, sizes)) in packings.iter().enumerate() {
        let name = format!("3D Packing {}", i);
        cube::plot(&positions, &sizes, &bricks[0], &name);
        cube::export(&positions, &sizes, &bricks[0], &name);
    }

    compute_distances(&packings);
    check_duality(&packings, &bricks[0]);

    println!("Time spent making packings: {:?} s", now.elapsed().as_secs());
}

struct Packing {
    positions: cube::Cube,
    sizes: cube::Cube,
    bricks: Vec<Point3D>,
    type_counts: Vec<Vec<HashMap<IntType, usize>>>,
}

impl Packing {
    fn new(brick: &Brick) -> Packing {
        Packing {
            positions: [[[Point3D::ZERO; N]; N]; N],
            sizes: [[[Point3D::ZERO; N]; N]; N],
            bricks: permutations(brick, N).iter().map(|permutation| {
                Point3D { x: permutation[0], y: permutation[1], z: permutation[2] }
            }).collect(),
            type_counts: repeat(
                repeat(HashMap::with_capacity(N)).take(N).collect() // For each level.
            ).take(N).collect() // For each dimension.
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
        && !cube::makes_sharp_corner(&self.positions, &self.sizes, &coord)
    }

    fn validate_type_count(&self, coord: &cube::Coord) -> bool {
        coord.iter().enumerate().all(|(i, &v)| self.type_counts[i][v].values().max().unwrap() <= &N)
    }

    fn decrement_type_count(&mut self, brick: &Point3D, coord: &cube::Coord) {
        for i in 0..N {
            let count = self.type_counts[i][coord[i]].entry(brick[i]).or_insert(0);
            *count -= 1;
        }
    }

    fn increment_type_count(&mut self, brick: &Point3D, coord: &cube::Coord) {
        for i in 0..N {
            let count = self.type_counts[i][coord[i]].entry(brick[i]).or_insert(0);
            *count += 1;
        }
    }
}

fn backtrack_cubes(bricks: &[Brick]) -> Vec<(cube::Cube, cube::Cube)> {
    let mut packings: Vec<Packing> = bricks.iter().map(|brick| Packing::new(brick)).collect();
    let coords = cube::make_coords();
    println!("Coords: {:?}", coords.len());
    let mut cubes = Vec::new();

    let mut records = [[[0; N]; N]; N];
    let mut i: usize = 0;
    let mut iteration: usize = 0;
    let mut successes: usize = 0;

    let max_tries = N * (N - 1) * (N - 2);

    loop {
        iteration += 1;
        if iteration % 100_000 == 0 {
            println!("Iteration {:?}, i: {:?}, successes: {:?}", iteration, i, successes);
        }

        let coord = coords[i];
        let (x, y, z) = (coord[0], coord[1], coord[2]);

        if records[x][y][z] < max_tries { // We'll try placing a brick.
            for packing in &mut packings {
                packing.place(&coord, records[x][y][z]);
            }
            records[x][y][z] += 1; // Register that this rotation has been tried.

            if packings.iter().all(|packing| packing.is_valid(&coord)) {
                if i == N * N * N - 1 { // We have successfully placed all bricks.
                    cubes.push((packings[0].positions, packings[0].sizes));
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
        for packing in &mut packings {
            packing.remove(&coords[i]);
        }
    }
    println!("Total iterations {:?}", iteration);
    cubes
}

fn compute_distances(packings: &Vec<(cube::Cube, cube::Cube)>) {
    for (i, &(_, a)) in packings.iter().enumerate() {
        print!("Packing {:2}: ", i);
        let distances = packings.iter().enumerate().map(|(_, &(_, b))| compute_distance(&a, &b)).collect::<Vec<_>>();
        let closest = packings.iter().enumerate().filter(|&(j, _)| i != j).map(|(_, &(_, b))| compute_distance(&a, &b)).min().unwrap();
        for (k, d) in distances.iter().enumerate().filter(|&(_, &d)| d == closest) {
            print!("({:2}, {:2}) ", k, d);
        }
        println!("Closest: {:2}", closest);

    }
}

fn compute_distance(a: &cube::Cube, b: &cube::Cube) -> usize {
    let coords = cube::make_coords();
    cube::symmetries(&b).iter().map(|&sizes| {
        coords.iter().filter(|&coord| {
            a[coord[0]][coord[1]][coord[2]] != sizes[coord[0]][coord[1]][coord[2]]
        }).count()
    }).min().unwrap()
}

fn check_duality(packings: &Vec<(cube::Cube, cube::Cube)>, brick: &Brick) {
    let permutations = vec!([2, 1, 0]);//permutations(&(0..N).collect::<Vec<_>>(), N);
    for permutation in &permutations {
        println!("Checking for dual using permutation {:?}:", permutation);
        let res = packings.iter().enumerate().map(|(i, &(_positions, sizes))| {
            if let Some(_) = apply_permutation(&sizes, permutation, brick) {
                format!("{}", i)
            } else {
                format!("")
            }
        }).filter(|s| s.len() > 0 ).collect::<Vec<_>>().join(", ");
        println!("{}", res);
    }
}

fn apply_permutation(sizes: &cube::Cube, permutation: &[usize], brick: &Brick) -> Option<(cube::Cube, cube::Cube)> {
    let mut map = HashMap::new();
    for (i, v) in brick.iter().enumerate() {
        map.insert(v, i);
    }
    let mut perm_sizes = [[[Point3D::ZERO; N]; N]; N];
    let mut perm_positions = [[[Point3D::ZERO; N]; N]; N];
    let coords = cube::make_coords();
    for coord in &coords {
        let (x, y, z) = (coord[0], coord[1], coord[2]);
        let size = sizes[x][y][z];
        let mut perm_size = Point3D::ZERO;
        for i in 0..N {
            perm_size[i] = brick[permutation[map[&size[i]]]];
        }
        perm_sizes[x][y][z] = perm_size;
        cube::position_brick(&mut perm_positions, &perm_sizes, &coord);
        if !cube::is_brick_valid(&perm_positions, &perm_sizes, &coord) {
            return None;
        }
    }
    Some((perm_positions, perm_sizes))
}
