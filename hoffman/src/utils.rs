use std::io::prelude::*;
use std::io::{BufReader, Error};
use std::fs::{File, create_dir_all};
use std::path::Path;
use std::collections::HashSet;

use combinatorics::*;
use super::{Coord, Shape, IntType};

pub fn make_coords(shape: &Shape) -> Vec<Coord> {
    let axes: Vec<Vec<usize>> = shape.iter().map(|&size| {
        (0..size).collect()
    }).collect();
    product(&axes)
}

pub fn list_has_unique_sums(list: &[IntType]) -> bool {
    let mut tuples: Vec<Vec<IntType>> = Vec::new();
    for i in 0..list.len() + 1 {
        tuples.extend(combinations(list, i));
    }
    let tuple_sums: Vec<IntType> = tuples.iter().map(|a| a.iter().sum()).collect();
    let unique_sums: HashSet<IntType> = tuple_sums.iter().cloned().collect();
    tuple_sums.len() == unique_sums.len()
}

pub fn list_except<T: PartialEq + Clone>(list: &[T], exclude: &[T]) -> Vec<T> {
    list.iter().filter(|&v| !exclude.contains(v)).cloned().collect()
}

pub fn write_file(contents: &String, directory: &String, file_name: &String) -> Result<(), Error> {
    // Create a path to the desired directory.
    let directory_path = Path::new(directory);

    // Create directory structure if it does not exist.
    create_dir_all(directory)?;

    // Create a path to the desired file.
    let file_path = directory_path.join(file_name);

    // Open file in write-only mode.
    let mut file = File::create(&file_path)?;

    // Write string to file.
    file.write_all(contents.as_bytes())
}

pub fn read_file(directory: &String, file_name: &String) -> Result<String, Error> {
    // Create a path to the desired file.
    let directory_path = Path::new(directory);
    let file_path = directory_path.join(file_name);

    // Open file.
    let file = File::open(&file_path)?;

    // Read file contents.
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}
