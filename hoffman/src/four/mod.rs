#[cfg(test)]
mod tests;

pub mod square;
pub mod cube;
pub mod tesseract;

use super::*;

pub const N: usize = 4;
pub const KERNEL_DIM: usize = 2;
pub type Brick = [IntType; N];
