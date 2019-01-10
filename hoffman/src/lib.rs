extern crate serde_json;
extern crate itertools;

pub mod ndarray;
pub mod utils;
pub mod combinatorics;
pub mod interval;
pub mod recipe;
pub mod recipe_builder;
pub mod plot;

#[cfg(test)]
mod tests;

pub use ndarray::*;
pub use interval::*;
pub use recipe::*;
pub use recipe_builder::*;

pub type IntType = i32;
pub type Coord = Vec<usize>;
pub type Shape = Vec<usize>;
pub type DimensionTuple = Vec<IntType>;
pub type Orientation = Vec<usize>;
pub type HyperRectangle = Vec<Interval>;
