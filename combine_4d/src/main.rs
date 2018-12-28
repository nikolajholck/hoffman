extern crate hoffman;

use hoffman::*;

use std::time::Instant;
use std::collections::HashMap;
use std::iter::repeat;

const N: usize = 4;

type Shape = Vec<usize>;

pub fn make_coords(shape: &Shape) -> Vec<Coord> {
    let axes: Vec<Vec<usize>> = shape.iter().map(|&size| {
        (0..size).collect()
    }).collect();
    combinatorics::product(&axes)
}

fn projection(coord: &Coord, projection: &Vec<usize>) -> Vec<usize> {
    projection.iter().map(|&i| coord[i]).collect()
}

fn permute_at(coord: &Coord, permutation: &Orientation, locations: &Vec<usize>) -> Coord {
    assert!(permutation.len() == locations.len(), "illegal sub-permutation");
    let mut result = coord.clone();
    for (i, &p) in permutation.iter().enumerate() {
        result[locations[i]] = coord[locations[p]];
    }
    result
}

#[derive(Clone)]
enum Container {
    One(Orientation),
    Many(HashMap<Coord, Group>)
}

#[derive(Clone)]
struct Group {
    contents: Container,
    shape: Shape
}

impl Group {

    fn permute(self, dimensions: &Vec<usize>, permutation: &Orientation, rotate: bool) -> Group {
        match self.contents {
            Container::One(orientation) => Group {
                contents: Container::One(permute_at(&orientation, permutation, dimensions)),
                shape: self.shape
            },
            Container::Many(groups) => {
                let mut new_groups: HashMap<Coord, Group> = HashMap::new();
                for (coord, group) in groups.into_iter() {
                    let new_group = group.permute(dimensions, permutation, rotate);
                    let new_coord = if rotate { permute_at(&coord, permutation, dimensions) } else { coord };
                    new_groups.insert(new_coord, new_group);
                }
                return Group {
                    contents: Container::Many(new_groups),
                    shape: permute_at(&self.shape, permutation, dimensions)
                }
            }
        }
    }

    fn solve(self, solution: &Recipe, dimensions: &Vec<usize>, rotate: bool) -> Group {
        if let Container::Many(groups) = self.contents {
            let mut new_groups: HashMap<Coord, Group> = HashMap::new();
            for (coord, group) in groups.into_iter() {
                let solution_coord = projection(&coord, dimensions);
                let permutation = &solution.map.get(&solution_coord).unwrap();
                let permuted_group = group.permute(dimensions, permutation, rotate);
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

    fn get_brick(&self, coord: &Coord, brick_counts: &Vec<usize>) -> Orientation {
        match &self.contents {
            Container::One(brick) => brick.clone(),
            Container::Many(ref groups) => {
                let recursive_brick_counts: Vec<usize> = (0..N).map(|i| brick_counts[i] / self.shape[i] ).collect();
                let local_coord: Coord = (0..N).map(|i| coord[i] / recursive_brick_counts[i] ).collect();
                let recursive_coord: Coord = (0..N).map(|i| coord[i] % recursive_brick_counts[i] ).collect();
                groups[&local_coord].get_brick(&recursive_coord, &recursive_brick_counts)
            }
        }
    }

    fn one(orientation: Orientation) -> Group {
        Group {
            contents: Container::One(orientation),
            shape: repeat(1).take(N).collect()
        }
    }

    fn many(contents: Vec<Group>, shape: &Shape) -> Group {
        let new_contents: HashMap<Coord, Group> = make_coords(shape).iter().zip(contents.iter()).map(|(coord, group)| {
            (coord.clone(), group.clone())
        }).collect();
        Group {
            contents: Container::Many(new_contents),
            shape: shape.clone()
        }
    }
}

fn main() {
    let now = Instant::now();

    let dimension_tuple = vec!(8, 9, 10, 12); // Wide
    //let dimension_tuple = vec!(10, 12, 13, 14); // Narrow

    let solution2d_one = Recipe {
        n: 2,
        m: 2,
        map: vec!(
            (vec!(0, 0), vec!(0, 1)),
            (vec!(0, 1), vec!(1, 0)),
            (vec!(1, 0), vec!(1, 0)),
            (vec!(1, 1), vec!(0, 1))
        ).into_iter().collect()
    };
    let solution2d_two = solution2d_one.pre_permute(&vec!(1, 0));

    let solutions = vec!(solution2d_one, solution2d_two);

    let solution_options = combinatorics::product(&repeat(solutions).take(2).collect::<Vec<_>>());
    println!("Solution options: {:?}", solution_options.len());

    let indices: Vec<usize> = (0..N).collect();
    let perms = combinatorics::permutations(&indices.as_slice(), N);

    let mut recipes: Vec<Recipe> = Vec::new();

    for solutions in &solution_options {
        for perm in &perms {
            let group = combine(perm, &solutions[0], &solutions[1]);
            recipes.push(convert_to_recipe(&group));
        }
    }

    for (i, recipe) in recipes.iter().enumerate() {
        let name = format!("4D Combined Packing {:?}", i);
        plot::plot_4d(recipe, &dimension_tuple, &name);
        recipe.save_json(&String::from("tesseracts"), &name);
    }
    println!("Total number of permuted recipes: {:?}", recipes.len());

    make_statistics(&recipes);
    check_duality(&recipes, &dimension_tuple);

    println!("Time spent making recipe: {:?} s", now.elapsed().as_secs());
}

fn combine(brick: &Orientation, solution_a: &Recipe, solution_b: &Recipe) -> Group {
    assert!(solution_a.n == solution_a.m);
    assert!(solution_b.n == solution_b.m);

    let (a, b) = (solution_a.n, solution_b.n);
    let ab = a * b;
    assert!(ab == N, "invalid combine dimensions.");

    // Generate bricks.
    let group_count = ab.pow(ab as u32);
    let mut groups: Vec<Group> = (0..group_count).map(|_| Group::one(brick.clone())).collect();

    for column in 0..a { // Solve columns
        //println!("Container count at column {} is {}", column, groups.len());
        let selected_dimensions: Vec<usize> = (0..b).map(|k| column + k * b).collect();
        let mut shape: Shape = repeat(1).take(N).collect();
        for &dim in &selected_dimensions {
            shape[dim] = a;
        }
        let group_size = a.pow(a as u32);

        let mut new_groups: Vec<Group> = Vec::new();
        for contents in groups.chunks(group_size) {
            let new_group = Group::many(contents.to_vec(), &shape).solve(solution_a, &selected_dimensions, false);
            new_groups.push(new_group);
        }
        groups = new_groups;
    }

    //println!("Box count after solving columns: {}", groups.len());

    for row in 0..b { // Solve rows
        //println!("Box count at row {} is, {}", row, groups.len());

        let selected_dimensions: Vec<usize> = (0..a).map(|k| row * b + k).collect();
        let mut shape: Shape = repeat(1).take(N).collect();
        for &dim in &selected_dimensions {
            shape[dim] = b;
        }
        let group_size = b.pow(b as u32);

        let mut new_groups: Vec<Group> = Vec::new();
        for contents in groups.chunks(group_size) {
            let new_group = Group::many(contents.to_vec(), &shape).solve(solution_b, &selected_dimensions, true);
            new_groups.push(new_group);
        }
        groups = new_groups;
    }

    //println!("Box count in the end: {}", groups.len());
    assert!(groups.len() == 1, "algorithm didn't result in packing.");
    groups.pop().unwrap()
}

fn convert_to_recipe(group: &Group) -> Recipe {
    let coords = utils::make_coords(N, N);
    let recipe_shape: Shape = repeat(N).take(N).collect();
    Recipe {
        n: N,
        m: N,
        map: coords.iter().map(|coord| {
            (coord.clone(), group.get_brick(coord, &recipe_shape))
        }).collect()
    }
}

fn check_duality(recipes: &Vec<Recipe>, dimension_tuple: &DimensionTuple) {
    let dims = (0..N).collect::<Vec<_>>();
    let permutations = combinatorics::permutations(&dims, N);
    for permutation in &permutations {
        println!("Checking for dual using permutation {:?}:", permutation);
        let res: usize = recipes.iter().filter(|recipe| {
            check_permutation(recipe, permutation, dimension_tuple)
        }).count();
        println!("Okay count: {}", res);
    }
}

fn check_permutation(recipe: &Recipe, permutation: &Orientation, dimension_tuple: &DimensionTuple) -> bool {
    let mut recipe_builder = RecipeBuilder::new(N, N, vec!(dimension_tuple.clone()));
    recipe_builder.produce(&recipe.pre_permute(permutation));
    recipe_builder.validate()
}

fn make_statistics(recipes: &Vec<Recipe>) {
    let dims = (0..N).collect::<Vec<_>>();
    for recipe in recipes {
        for dim in 0..N {
            for level in 0..N {
                let mut template_coord = [0; N];
                template_coord[dim] = level;
                let other_dims = utils::list_except(&dims, &[dim]);
                let mut axes: Vec<Vec<usize>> = [N; N-1].iter().map(|&size| {
                    (0..size).collect()
                }).collect();
                axes.insert(dim, vec!(level));
                let coords = combinatorics::product(&axes);
                let mut type_map = HashMap::with_capacity(N);
                let mut orientations_map: HashMap<usize, HashMap<usize, usize>> = HashMap::with_capacity(N);
                for coord in &coords {
                    let perm = recipe.map.get(coord).unwrap();
                    let type_key = perm[dim];
                    let count = type_map.entry(type_key).or_insert(0);
                    *count += 1;

                    let orientation_map = orientations_map.entry(type_key).or_insert(HashMap::new());
                    let orientation = [perm[other_dims[0]], perm[other_dims[1]], perm[other_dims[2]]];
                    let orientations = combinatorics::permutations(&utils::list_except(&dims, &[type_key]), N - 1);
                    let mut idx: Option<_> = None;
                    for (i, ori) in orientations.iter().enumerate() {
                        if ori[..N-1].iter().zip(orientation.iter()).all(|(a, b)| a == b) {
                            idx = Some(i);
                            break;
                        }
                    }
                    let idx = idx.expect(&format!("Could not find perm type {:?}, {:?}, {:?}", orientation, &utils::list_except(&dims, &[type_key]), orientations));
                    let orientation_count = orientation_map.entry(idx).or_insert(0);
                    *orientation_count += 1;
                }
                println!("Orientation map: {:?}", orientations_map.iter().map(|(_, map)| {
                    map.iter().map(|(_, &c)| c).collect::<Vec<usize>>()
                }).collect::<Vec<Vec<usize>>>());
                println!("Orientation map: {:?}", orientations_map);
            }
        }
        break
    }
}
