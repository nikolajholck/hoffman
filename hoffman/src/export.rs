use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use super::*;

pub struct Brick {
    pub coord: Vec<usize>,
    pub position: Vec<IntType>,
    pub size: Vec<IntType>
}

pub struct Export {
    pub name: Option<String>,
    pub dimensions: usize,
    pub brick: Vec<IntType>,
    pub bricks: Vec<Brick>
}

impl Brick {
    fn to_json(&self, brick: &Vec<IntType>) -> String {
        let coord = format!("[{}]", self.coord.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", "));
        let position = format!("[{}]", self.position.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", "));
        let size = format!("[{}]", self.size.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", "));
        let permutation = format!("[{}]", self.size.iter().map(|&v| brick.iter().position(|&b| b == v).unwrap().to_string() ).collect::<Vec<_>>().join(", "));
        format!("{{ \"coord\": {}, \"position\": {}, \"size\": {}, \"permutation\": {} }}", coord, position, size, permutation)
    }
}

impl Export {
    fn to_json(&self) -> String {
        let name = self.name.clone().unwrap_or(String::from(""));
        let dimensions = self.dimensions.to_string();
        let brick = format!("[{}]", self.brick.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", "));
        let bricks = format!("[{}]", self.bricks.iter().map(|brick| brick.to_json(&self.brick)).collect::<Vec<_>>().join(",\n"));
        format!("{{ \"name\": \"{}\", \"dimensions\": {}, \"brick\": {}, \"bricks\": {} }}", name, dimensions, brick, bricks)
    }

    pub fn save(&self, filename: &String) {
        // Create a path to the desired file.
        let path_str = format!("exports/{}.json", filename.clone());
        let path = Path::new(&path_str);
        let display = path.display();

        // Open file in write-only mode, returns `io::Result<File>`.
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why.description()),
            Ok(file) => file
        };

        // Generate json.
        let json = self.to_json();

        // Write string to `file`, returns `io::Result<()>`.
        match file.write_all(json.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
            Ok(_) => return
        }
    }
}
