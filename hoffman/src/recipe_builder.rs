use std::collections::HashMap;
use std::cmp::min;
use std::iter::repeat;

use utils::*;
use combinatorics::*;
use plot::*;
use super::*;

pub struct RecipeBuilder {
    n: usize,
    m: usize,
    dimension_tuples: Vec<DimensionTuple>,
    recipe: Recipe,
    packings: HashMap<Coord, Vec<HyperRectangle>>,
    subgrid_counts: Vec<Vec<HashMap<usize, usize>>>
}

impl RecipeBuilder {

    pub fn new(n: usize, m: usize, dimension_tuples: Vec<DimensionTuple>) -> RecipeBuilder {
        let number_of_dimension_tuples = n.pow(m as u32);
        RecipeBuilder {
            n: n,
            m: m,
            dimension_tuples: dimension_tuples,
            recipe: Recipe::new(n, m),
            packings: HashMap::with_capacity(number_of_dimension_tuples),
            subgrid_counts: repeat(
                repeat(HashMap::with_capacity(n)).take(n).collect() // For each level.
            ).take(m).collect() // For each dimension.
        }
    }

    pub fn get_recipe(&self) -> &Recipe {
        &self.recipe
    }

    pub fn get_hyper_rectangles(&self, coord: &Coord) -> &Vec<HyperRectangle> {
        self.packings.get(coord).unwrap()
    }

    pub fn produce(&mut self, recipe: &Recipe) {
        assert!(self.packings.is_empty());
        assert!(self.n == recipe.n);
        assert!(self.m == recipe.m);
        let coords = utils::make_coords(self.n, self.m);
        for coord in &coords {
            match recipe.map.get(coord) {
                Some(orientation) => self.insert(coord, orientation),
                None => continue
            }
        }
    }

    pub fn validate(&self) -> bool {
        let coords = utils::make_coords(self.n, self.m);
        for coord in &coords {
            if !self.is_valid(coord) {
                return false
            }
        }
        true
    }

    pub fn insert(&mut self, coord: &Coord, orientation: &Orientation) {
        self.recipe.map.insert(coord.clone(), orientation.clone());
        self.increment_subgrid_count(coord, orientation);

        let hyper_rectangles = {
            let foundations = (0..self.m).map(|dim| {
                if coord[dim] == 0 { return None }
                let mut index = coord.clone();
                index[dim] -= 1;
                self.packings.get(&index)
            }).collect::<Vec<_>>();
            self.dimension_tuples.iter().enumerate().map(|(i, dimension_tuple)| {
                foundations.iter().enumerate().map(|(dim, foundation)| {
                    let begin = match foundation {
                        Some(hyper_rectangles) => hyper_rectangles[i][dim].end,
                        None => 0
                    };
                    Interval {
                        begin: begin,
                        end: begin + dimension_tuple[orientation[dim]]
                    }
                }).collect()
            }).collect()
        };

        self.packings.insert(coord.clone(), hyper_rectangles);
    }

    pub fn remove(&mut self, coord: &Coord) {
        let orientation = self.recipe.map.remove(coord).unwrap();
        self.decrement_subgrid_count(coord, &orientation);
        self.packings.remove(coord);
    }

    pub fn is_valid(&self, coord: &Coord) -> bool {
        self.satisfies_line_criterion(coord)
        && !self.has_overlaps(coord)
        && !self.is_sharp_corner(coord)
        && self.satisfies_subgrid_criterion(coord)
    }

    pub fn satisfies_line_criterion(&self, coord: &Coord) -> bool {
        let orientation = &self.recipe.map.get(coord).unwrap();

        for (dim, &c) in coord.iter().enumerate() {
            let mut index = coord.clone();
            for j in 0..c {
                index[dim] = j;

                match self.recipe.map.get(&index) {
                    Some(other) => if other[dim] == orientation[dim] { return false },
                    None => continue
                };
            }
        }
        true
    }

    pub fn satisfies_subgrid_criterion(&self, coord: &Coord) -> bool {
        let limit = self.n.pow(self.n as u32 - 2);
        coord.iter().enumerate().all(|(dim, &v)| {
            self.subgrid_counts[dim][v].values().max().unwrap() <= &limit
        })
    }

    fn decrement_subgrid_count(&mut self, coord: &Coord, orientation: &Orientation) {
        for dim in 0..self.m {
            let count = self.subgrid_counts[dim][coord[dim]].entry(orientation[dim]).or_insert(0);
            *count -= 1;
        }
    }

    fn increment_subgrid_count(&mut self, coord: &Coord, orientation: &Orientation) {
        for dim in 0..self.m {
            let count = self.subgrid_counts[dim][coord[dim]].entry(orientation[dim]).or_insert(0);
            *count += 1;
        }
    }

    pub fn has_overlaps(&self, coord: &Coord) -> bool {
        let hyper_rectangles = &self.packings.get(coord).unwrap();

        // Construct neighbouring coordinates.
        let neighbourhood: Vec<Vec<usize>> = coord.iter().map(|&v| {
            let before = if v == 0 { 0 } else { v - 1 };
            let after = min(v + 2, self.n);
            (before..after).collect()
        }).collect();
        let neighbours = product(neighbourhood.as_slice());

        // Check for overlap.
        for neighbour in &neighbours {
            if neighbour == coord || !self.packings.contains_key(neighbour) { continue }
            let other_hyper_rectangles = &self.packings.get(neighbour).unwrap();

            // Check if any of the packings has an overlap.
            if hyper_rectangles.iter().zip(other_hyper_rectangles.iter()).any(|(hyper_rectangle, other_hyper_rectangle)| {
                hyper_rectangle.iter().zip(other_hyper_rectangle.iter()).all(|(a, b)| a.intersects(b) )
            }) {
                return true
            }
        }
        false
    }

    pub fn is_sharp_corner(&self, coord: &Coord) -> bool {
        let this_hyper_rectangles = &self.packings.get(coord).unwrap();

        let possible_directions: Vec<usize> = (0..self.m).filter(|&i| coord[i] > 0).collect();
        let direction_count = possible_directions.len();
        for dimensionality in 1..=direction_count {
            let direction_combinations = combinations(&possible_directions, dimensionality);
            for directions in &direction_combinations {
                let mut foundation_coord = coord.clone();
                for &direction in directions {
                    foundation_coord[direction] -= 1;
                }
                let foundation_hyper_rectangles = match self.packings.get(&foundation_coord) {
                    Some(hyper_rectangles) => hyper_rectangles,
                    None => continue
                };
                let other_dims = list_except(&(0..self.m).collect::<Vec<_>>(), &directions);

                for &dim in &other_dims {
                    if coord[dim] + 1 >= self.n { continue }

                    let other_coords: Vec<Coord> = directions.iter().map(|&direction| {
                        let mut other_coord = coord.clone();
                        other_coord[dim] += 1;
                        other_coord[direction] -= 1;
                        other_coord
                    }).collect();

                    if other_coords.iter().any(|coord| !self.packings.contains_key(coord)) { continue }

                    let other_hyper_rectangles: Vec<&Vec<HyperRectangle>> = other_coords.iter().map(|coord| {
                        self.packings.get(coord).unwrap()
                    }).collect();

                    for i in 0..self.dimension_tuples.len() {
                        if foundation_hyper_rectangles[i][dim].end > this_hyper_rectangles[i][dim].end
                        && directions.iter().zip(other_hyper_rectangles.iter()).all(|(&dir, other)| {
                            foundation_hyper_rectangles[i][dir].end > other[i][dir].end
                        }) {
                            return true
                        }
                    }
                }

            }
        }
        false
    }

    pub fn get_rectangles_at(&self, fixed: Vec<(usize, usize)>) -> Vec<Rectangle> {
        let fixed_dims: Vec<usize> = fixed.iter().map(|&(dim, _)| dim).collect();
        assert!(fixed.len() + 2 == self.m, "Can only plot in 2D.");
        let mut rects = Vec::new();
        for i in 0..self.n {
            for j in 0..self.n {
                let mut index = vec!(i, j);
                for &(dim, level) in &fixed {
                    index.insert(dim, level);
                }
                let varying_dims = utils::list_except(&(0..self.m).collect::<Vec<_>>(), &fixed_dims);
                match self.packings.get(&index) {
                    Some(hyper_rectangles) => {
                        let hyper_rectangle = &hyper_rectangles[0];
                        let rectangle = plot::Rectangle {
                            x: hyper_rectangle[varying_dims[0]].begin,
                            y: hyper_rectangle[varying_dims[1]].begin,
                            width: hyper_rectangle[varying_dims[0]].width(),
                            height: hyper_rectangle[varying_dims[1]].width()
                        };
                        rects.push(rectangle);
                    },
                    None => continue
                }

            }
        }
        rects
    }
}
