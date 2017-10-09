extern crate hoffman;

use hoffman::*;
use hoffman::four::*;

use std::time::Instant;
use std::collections::HashMap;

fn make_coords(shape: Shape) -> Vec<Coord> {
    let axes: Vec<Vec<usize>> = shape.iter().map(|&size| {
        (0..size).collect()
    }).collect();
    product(&axes).iter().map(|list| {
        let mut coord = [0; N];
        for i in 0..N {
            coord[i] = list[i];
        }
        coord
    }).collect()
}

fn projection(coord: &Coord, projection: &[usize]) -> [usize; 2] {
    assert!(coord.len() >= projection.len(), "illegal projection");
    let mut result = [0; 2];
    for i in 0..2 {
        result[i] = coord[projection[i]];
    }
    result
}

fn permute_at<T: Copy>(coord: &[T; N], permutation: &[usize], locations: &[usize]) -> [T; N] {
    assert!(permutation.len() == locations.len(), "illegal sub-permutation");
    let mut result = coord.clone();
    for (i, &p) in permutation.iter().enumerate() {
        result[locations[i]] = coord[locations[p]];
    }
    result
}

type Coord = [usize; N];
type Shape = [usize; N];

#[derive(Clone)]
enum Container {
    One(Brick),
    Many(HashMap<Coord, Group>)
}

#[derive(Clone)]
struct Group {
    contents: Container,
    shape: Shape
}

impl Group {

    fn permute(self, dimensions: &[usize], permutation: &[usize], rotate: bool) -> Group {
        match self.contents {
            Container::One(brick) => Group {
                contents: Container::One(permute_at(&brick, &permutation, &dimensions)),
                shape: self.shape
            },
            Container::Many(groups) => {
                let mut new_groups: HashMap<Coord, Group> = HashMap::new();
                for (coord, group) in groups.into_iter() {
                    let new_group = group.permute(&dimensions, &permutation, rotate);
                    let new_coord = if rotate { permute_at(&coord, &permutation, &dimensions) } else { coord };
                    new_groups.insert(new_coord, new_group);
                }
                return Group {
                    contents: Container::Many(new_groups),
                    shape: permute_at(&self.shape, &permutation, &dimensions)
                }
            }
        }
    }

    fn solve(self, solution: &HashMap<[usize; 2], [usize; 2]>, dimensions: &[usize], rotate: bool) -> Group {
        if let Container::Many(groups) = self.contents {
            let mut new_groups: HashMap<Coord, Group> = HashMap::new();
            for (coord, group) in groups.into_iter() {
                let solution_coord = projection(&coord, dimensions);
                let permutation = solution[&solution_coord];
                let permuted_group = group.permute(dimensions, &permutation, rotate);
                new_groups.insert(coord, permuted_group);
            }
            return Group {
                contents: Container::Many(new_groups),
                shape: self.shape
            }
        } else {
            panic!("cannot solve a single brick!");
        }
    }

    fn get_brick(&self, coord: &Coord, brick_counts: &Shape) -> Point4D {
        match self.contents {
            Container::One(brick) => Point4D { x: brick[0], y: brick[1], z: brick[2], w: brick[3] },
            Container::Many(ref groups) => {
                let mut local_coord = [0; N];
                let mut recursive_coord = [0; N];
                let mut recursive_brick_counts = [0; N];
                for i in 0..N {
                    recursive_brick_counts[i] = brick_counts[i] / self.shape[i];
                    local_coord[i] = coord[i] / recursive_brick_counts[i];
                    recursive_coord[i] = coord[i] % recursive_brick_counts[i];
                }
                groups[&local_coord].get_brick(&recursive_coord, &recursive_brick_counts)
            }
        }
    }

    fn one(brick: Brick) -> Group {
        Group {
            contents: Container::One(brick),
            shape: [1; N]
        }
    }

    fn many(contents: Vec<Group>, shape: Shape) -> Group {
        let mut new_contents: HashMap<Coord, Group> = HashMap::new();
        for (&coord, group) in make_coords(shape).iter().zip(contents.into_iter()) {
            new_contents.insert(coord, group);
        }
        Group {
            contents: Container::Many(new_contents),
            shape: shape
        }
    }

}

fn main() {
    let now = Instant::now();

    // let brick = [53, 54, 57, 59]; // 10 (wide)
    let brick = [57, 59, 62, 63]; // 10 (narrow)
    //let brick = [1, 2, 4, 7]; // testing

    let solution2d_one: HashMap<[usize; 2], [usize; 2]> =
    [([0, 0], [0, 1]),
     ([0, 1], [1, 0]),
     ([1, 0], [1, 0]),
     ([1, 1], [0, 1])]
     .iter().cloned().collect();
     let solution2d_two: HashMap<[usize; 2], [usize; 2]> =
     [([0, 0], [1, 0]),
      ([0, 1], [0, 1]),
      ([1, 0], [0, 1]),
      ([1, 1], [1, 0])]
      .iter().cloned().collect();
    let solutions = vec!(solution2d_one, solution2d_two);

    let solution_options = product(&[solutions.clone(), solutions.clone()]);
    println!("Solution options: {:?}", solution_options.len());

    let brick_options: Vec<Brick> = permutations(&brick, N).iter().map(|list| {
        let mut new_brick = [0; N];
        for i in 0..N {
            new_brick[i] = list[i];
        }
        new_brick
    }).collect();

    let comparator = Comparator::constructor(&brick);

    let mut packings: Vec<(tesseract::Tesseract, tesseract::Tesseract)> = Vec::new();

    for solutions in &solution_options {
        for current_brick in &brick_options {
            let group = combine(current_brick, &solutions[0], &solutions[1]);
            packings.push(convert_to_packing(&group, &comparator));
        }
    }

    for (i, &(positions, sizes)) in packings.iter().enumerate() {
        let name = format!("4D Combined Packing {:?}", i);
        tesseract::plot(&positions, &sizes, &brick, &name);
    }

    println!("Time spent making packing: {:?} s", now.elapsed().as_secs());

}

fn combine(brick: &Brick, solution_a: &HashMap<[usize; 2], [usize; 2]>, solution_b: &HashMap<[usize; 2], [usize; 2]>) -> Group {
    let (a, b) = (2, 2);
    let ab = 4;
    assert!(ab == N, "invalid combine dimensions.");

    // Generate bricks.
    let group_count = ab.pow(ab as u32);
    let mut groups: Vec<Group> = (0..group_count).map(|_| Group::one(brick.clone())).collect();

    for column in 0..a { // Solve columns
        println!("Container count at column {} is {}", column, groups.len());
        let selected_dimensions: Vec<usize> = (0..b).map(|k| column + k * b).collect();
        let mut shape = [1; N];
        for &dim in &selected_dimensions {
            shape[dim] = a;
        }
        let group_size = a.pow(a as u32);

        let mut new_groups: Vec<Group> = Vec::new();
        for contents in groups.chunks(group_size) {
            let new_group = Group::many(contents.to_vec(), shape).solve(&solution_a, &selected_dimensions, false);
            new_groups.push(new_group);
        }
        groups = new_groups;
    }

    println!("Box count after solving columns: {}", groups.len());

    for row in 0..b { // Solve rows
        println!("Box count at row {} is, {}", row, groups.len());

        let selected_dimensions: Vec<usize> = (0..a).map(|k| row * b + k).collect();

        let mut shape = [1; N];
        for &dim in &selected_dimensions {
            shape[dim] = b;
        }
        let group_size = b.pow(b as u32);

        let mut new_groups: Vec<Group> = Vec::new();
        for contents in groups.chunks(group_size) {
            let new_group = Group::many(contents.to_vec(), shape).solve(&solution_b, &selected_dimensions, true);
            new_groups.push(new_group);
        }
        groups = new_groups;
    }

    println!("Box count in the end: {}", groups.len());
    assert!(groups.len() == 1, "algorithm didn't result in packing.");
    groups.pop().unwrap()
}

fn convert_to_packing(group: &Group, comparator: &Comparator) -> (tesseract::Tesseract, tesseract::Tesseract) {
    let coords = make_coords([N, N, N, N]);
    println!("Coords: {:?}", coords.len());

    let mut sizes = [[[[Point4D::ZERO; N]; N]; N]; N];
    let mut positions = [[[[Point4D::ZERO; N]; N]; N]; N];

    for coord in coords.iter() {
        let (x, y, z, w) = (coord[0], coord[1], coord[2], coord[3]);
        sizes[x][y][z][w] = group.get_brick(&coord, &[N, N, N, N]);
        tesseract::position_brick(&mut positions, &sizes, &coord);
        if !tesseract::is_brick_valid(&positions, &sizes, &coord, &comparator) {
            println!("Error: Something is wrong with the packing at {:?}", coord);
        }
    }
    (positions, sizes)
}
