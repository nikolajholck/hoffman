use std::iter::repeat;
use serde_json::Value;

use super::*;
use combinatorics::*;

#[derive(Clone, PartialEq)]
pub struct Recipe {
    pub n: usize,
    pub m: usize,
    pub map: NdArray<Orientation>
}

impl Recipe {
    pub fn new(n: usize, m: usize) -> Recipe {
        let shape = repeat(n).take(m).collect();
        Recipe {
            n: n,
            m: m,
            map: NdArray::new(&shape)
        }
    }

    pub fn is_self_symmetric(&self) -> bool {
        self.symmetries()[1..].contains(self)
    }

    pub fn symmetries(&self) -> Vec<Recipe> {
        let mut symmetries = Vec::new();
        let dims: Vec<usize> = (0..self.m).collect();
        let directions = [true, false];
        let direction_choices = combinations_with_repetition(&directions, self.m);
        let axis_permutations = permutations(&dims, self.m);
        for axes in &axis_permutations {
            for directions in &direction_choices {
                symmetries.push(self.symmetry(axes, directions));
            }
        }
        assert!(symmetries.len() == (2 as usize).pow(self.m as u32) * factorial(self.m) "Invalid number of symmetries.");
        symmetries
    }

    fn symmetry(&self, axis_permutation: &Vec<usize>, axis_directions: &Vec<bool>) -> Recipe {
        Recipe {
            n: self.n,
            m: self.m,
            map: self.map.map(|(coord, orientation)| {
                let transformed_coord = axis_directions.iter().zip(axis_permutation.iter()).map(|(&forward, &i)| {
                    if forward { coord[i] } else { (self.n - 1) - coord[i] }
                }).collect();
                let transformed_orientation = axis_permutation.iter().map(|&i| orientation[i]).collect();
                (transformed_coord, transformed_orientation)
            })
        }
    }

    pub fn find_unique(recipes: Vec<Recipe>) -> Vec<Recipe> {
        let mut unique: Vec<Recipe> = Vec::new();
        for suspect_recipe in recipes {
            let symmetries = suspect_recipe.symmetries();
            if unique.iter().all(|recipe| !symmetries.contains(&recipe)) {
                unique.push(suspect_recipe)
            }
        }
        unique
    }

    pub fn distance_to(&self, other: &Recipe) -> usize {
        self.symmetries().iter().map(|symmetry| {
            symmetry.map.differences(&other.map)
        }).min().unwrap()
    }

    pub fn pre_permute(&self, permutation: &Orientation) -> Recipe {
        Recipe {
            n: self.n,
            m: self.m,
            map: self.map.map(|(coord, old_orientation)| {
                (coord.clone(), old_orientation.iter().map(|&i| permutation[i]).collect())
            })
        }
    }

}

impl Recipe {
    pub fn save_json(&self, directory: &String, file_name: &String) {
        let n = self.n.to_string();
        let m = self.m.to_string();
        let map = format!("[{}]", self.map.iter()
        .map(|(coord, orientation)| {
            let coord = format!("[{}]", coord.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", "));
            let orientation = format!("[{}]", orientation.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", "));
            format!("{{ \"coord\": {}, \"permutation\": {} }}", coord, orientation)
        }).collect::<Vec<_>>().join(",\n"));
        let json = format!("{{ \"n\": {}, \"m\": {}, \"map\": {} }}", n, m, map);
        utils::write_file(&json, &format!("exports/{}", directory), &format!("{}.json", file_name))
          .expect("Error writing recipe to file");
    }

    pub fn load_json(directory: &String, file_name: &String) -> Recipe {
        let json = utils::read_file(directory, &format!("{}.json", file_name))
          .expect("Error reading recipe from file.");
        let recipe_json: Value = serde_json::from_str(&json)
          .expect("Error reading recipe from file.");

        let n = recipe_json["n"].as_u64().unwrap() as usize;
        let m = recipe_json["m"].as_u64().unwrap() as usize;
        let xs: Vec<Value> = recipe_json["map"].as_array().unwrap().to_vec();

        let shape = repeat(n).take(m).collect();
        let mut map = NdArray::new(&shape);
        for x in &xs {
            let kv = x.as_object().unwrap();
            let coord: Coord = kv["coord"].as_array().unwrap().iter().map(|v| v.as_u64().unwrap() as usize).collect();
            let permutation: Orientation = kv["permutation"].as_array().unwrap().iter().map(|v| v.as_u64().unwrap() as usize).collect();
            map.insert(&coord, permutation);
        }
        Recipe {
            n: n,
            m: m,
            map: map
        }
    }
}
