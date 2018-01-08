extern crate hoffman;
extern crate serde_json;

use hoffman::*;
use hoffman::four::*;
use std::collections::HashMap;
use std::time::Instant;

use serde_json::{Value};
use std::fs::File;

fn main() {
    let maps = import_packings();
    let coords = cube::make_coords([N; cube::M]);
    let limit = 50;
    let start = Instant::now();
    for a in 1..limit {
        for b in a..limit {
            let sub_time = Instant::now();
            for c in b..limit {
                for d in c..limit {
                    let brick = [a, b, c, d];
                    let packings = generate_packings(&maps, &brick);
                    for &(positions, sizes) in packings.iter() {
                        for i in 0..N * N * N {
                            for j in i + 1..N * N * N {
                                if cube::bricks_intersect(&positions, &sizes, &coords[i], &coords[j]) {
                                    panic!("Counter-example using dimension tuple {:?}!", brick);
                                }
                            }
                        }
                    }
                }
            }
            println!("({}, {}, _, _) dimension tuples passed in {} seconds.", a, b, sub_time.elapsed().as_secs());
        }
    }
    println!("All dimension tuples passed in {} seconds.", start.elapsed().as_secs());
}

fn import_packings() -> Vec<HashMap<(usize, usize, usize), Vec<usize>>> {
    let mut maps: Vec<HashMap<(usize, usize, usize), Vec<usize>>> = Vec::new();
    for i in 0..900 {
        let file = File::open(format!("res/cube-4d-{}.json", i + 1)).unwrap();
        let json: Value = serde_json::from_reader(file).unwrap();
        let bricks: Vec<Value> = json["bricks"].as_array().unwrap().to_vec();
        let map: HashMap<(usize, usize, usize), Vec<usize>> = bricks.iter().map(|ref brick| {
            let map = brick.as_object().unwrap();
            let c: Vec<usize> = map["coord"].as_array().unwrap().iter().map(|ref v| v.as_u64().unwrap() as usize).collect::<_>();
            let permutation: Vec<usize> = map["permutation"].as_array().unwrap().iter().map(|ref v| v.as_u64().unwrap() as usize).collect::<_>();
            let coord = (c[0], c[1], c[2]);
            (coord, permutation)
        }).collect::<_>();
        maps.push(map);
    }
    maps
}

fn permute(brick: &Brick, permutation: &[usize]) -> [IntType; cube::M] {
    let mut result = [0; cube::M];
    for i in 0..cube::M {
        result[i] = brick[permutation[i]];
    }
    result
}

fn generate_packings(maps: &Vec<HashMap<(usize, usize, usize), Vec<usize>>>, brick: &Brick) -> Vec<(cube::Cube, cube::Cube)> {
    let coords = cube::make_coords([N; cube::M]);
    let mut packings: Vec<(cube::Cube, cube::Cube)> = Vec::new();
    for map in maps.iter() {
        let mut positions = [[[Point3D::ZERO; N]; N]; N];
        let mut sizes = [[[Point3D::ZERO; N]; N]; N];
        for coord in &coords {
            let (x, y, z) = (coord[0], coord[1], coord[2]);
            let permutation = &map[&(x, y, z)];
            let size = permute(brick, permutation);
            sizes[x][y][z] = Point3D { x: size[0], y: size[1], z: size[2] };
            cube::position_brick(&mut positions, &sizes, coord);
        }
        packings.push((positions, sizes))
    }
    packings
}
