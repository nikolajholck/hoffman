use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;

use super::*;

const COLORS: &'static [&'static str] = &[
    "rgb(236, 31, 38)",  // Red
    "rgb(121, 193, 68)", // Green
    "rgb(0, 125, 199)",  // Blue
    "rgb(244, 112, 37)", // Orange
    "rgb(252, 223, 7)",  // Yellow
    "rgb(138, 40, 143)", // Violet
];
const MARGIN: f64 = 40.0;
const WIDTH: f64 = 800.0;

#[derive(Clone)]
pub struct Rectangle {
    pub x: IntType,
    pub y: IntType,
    pub width: IntType,
    pub height: IntType
}

#[derive(Clone)]
pub struct Plot {
    pub name: Option<String>,
    pub rectangles: Vec<Rectangle>
}

pub struct Figure {
    pub name: Option<String>,
    pub plots: Vec<Plot>,
    pub brick: Vec<IntType>,
    pub rows: usize,
    pub columns: usize
}

impl Rectangle {
    fn to_svg(&self, colors: &HashMap<IntType, usize>) -> String {
        if self.width == 0 || self.height == 0 { return format!("") }
        format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" style=\"fill:{};stroke:#000;stroke-width:1;\" vector-effect=\"non-scaling-stroke\" />",
        self.x, self.y, self.width, self.height, COLORS[colors[&(self.width + self.height)]])
    }
}

impl Plot {
    fn to_svg(&self, colors: &HashMap<IntType, usize>) -> String {
        self.rectangles.iter().map(|rect| rect.to_svg(colors)).collect::<Vec<String>>().join("\n")
    }
}

impl Figure {
    fn to_svg(&self) -> String {
        assert!(self.plots.len() == self.rows * self.columns, "Number of plots doesn't match number of rows and columns.");

        let mut colors: HashMap<IntType, usize> = HashMap::new();
        for (i, sum) in combinations(&self.brick, 2).iter().map(|li| li.iter().sum()).enumerate() {
            colors.insert(sum, i);
        }

        let mut svg = String::new();
        let brick_sum: IntType = self.brick.iter().sum();
        let plot_size = (WIDTH - MARGIN * ((self.columns + 1) as f64)) / self.columns as f64;
        let plot_scale = plot_size / brick_sum as f64;
        let figure_height = MARGIN * (self.rows + 1) as f64 + plot_size * self.rows as f64;

        svg.push_str(&format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n", WIDTH, figure_height, WIDTH, figure_height));
        for row in 0..self.rows {
            for column in 0..self.columns {
                let x = (MARGIN + plot_size) * column as f64 + MARGIN;
                let y = (MARGIN + plot_size) * row as f64 + MARGIN;
                let plot = &self.plots[row * self.columns + column];
                let name = plot.name.clone().unwrap_or(String::from(""));
                svg.push_str(&format!("<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-size=\"20\">{}</text>", x + plot_size * 0.5, y - 10.0, name));
                svg.push_str(&format!("<g transform=\"matrix({} 0 0 {} {} {})\">\n", plot_scale, plot_scale, x, y));
                svg.push_str(&format!("<rect x=\"0\" y=\"0\" width=\"{}\" height=\"{}\" style=\"fill:none;stroke:#000;stroke-width:1;\" vector-effect=\"non-scaling-stroke\" />", brick_sum, brick_sum));
                svg.push_str(&format!("{}\n", plot.to_svg(&colors)));
                svg.push_str(&format!("</g>\n"));
            }
        }
        svg.push_str("</svg>");
        svg
    }

    pub fn save(&self, filename: &String) {

        // Create a path to the desired file.
        let path_str = format!("plots/{}.svg", filename.clone());
        let path = Path::new(&path_str);
        let display = path.display();

        // Open file in write-only mode, returns `io::Result<File>`.
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why.description()),
            Ok(file) => file
        };

        // Generate SVG.
        let svg = self.to_svg();

        // Write string to `file`, returns `io::Result<()>`.
        match file.write_all(svg.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
            Ok(_) => return
        }
    }
}
