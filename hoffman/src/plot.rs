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
const TIKZ_COLORS: &'static [&'static str] = &[
    "custom-red",  // Red
    "custom-green", // Green
    "custom-blue",  // Blue
    "custom-orange", // Orange
    "custom-yellow",  // Yellow
    "custom-violet", // Violet
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

    fn to_tikz(&self, colors: &HashMap<IntType, usize>) -> String {
        if self.width == 0 || self.height == 0 { return format!("") }
        format!("\\filldraw[fill={}, draw=black] ({},{}) rectangle ({},{});",
        TIKZ_COLORS[colors[&(self.width + self.height)]], self.x, self.y, self.x + self.width, self.y + self.height)
    }
}

impl Plot {
    fn to_svg(&self, colors: &HashMap<IntType, usize>) -> String {
        self.rectangles.iter().map(|rect| rect.to_svg(colors)).collect::<Vec<String>>().join("\n")
    }

    fn to_tikz(&self, colors: &HashMap<IntType, usize>) -> String {
        self.rectangles.iter().map(|rect| rect.to_tikz(colors)).collect::<Vec<String>>().join("\n")
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
                svg.push_str(&format!("<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-size=\"16\">{}</text>", x + plot_size * 0.5, y - 10.0, name));
                svg.push_str(&format!("<g transform=\"matrix({} 0 0 {} {} {})\">\n", plot_scale, plot_scale, x, y));
                svg.push_str(&format!("<rect x=\"0\" y=\"0\" width=\"{}\" height=\"{}\" style=\"fill:none;stroke:#000;stroke-width:1;\" vector-effect=\"non-scaling-stroke\" />", brick_sum, brick_sum));
                svg.push_str(&format!("{}\n", plot.to_svg(&colors)));
                svg.push_str(&format!("</g>\n"));
            }
        }
        svg.push_str("</svg>");
        svg
    }

    fn to_tikz(&self) -> String {
        assert!(self.plots.len() == self.rows * self.columns, "Number of plots doesn't match number of rows and columns.");

        let mut colors: HashMap<IntType, usize> = HashMap::new();
        for (i, sum) in combinations(&self.brick, 2).iter().map(|li| li.iter().sum()).enumerate() {
            colors.insert(sum, i);
        }

        let mut tikz = String::new();
        let brick_sum: IntType = self.brick.iter().sum();
        let figure_scale = 1.0 / (self.columns as f64) - 0.03;
        let text_width = 12.0;
        let tikz_scale = figure_scale * text_width / brick_sum as f64;

        let rows = (0..self.rows).map(|row|
            (0..self.columns).map(|column| {
                let plot = &self.plots[row * self.columns + column];
                let mut subfigure = String::new();
                subfigure.push_str(&format!("    \\begin{{subfigure}}[b]{{{:.3}\\textwidth}}\n", figure_scale));
                subfigure.push_str(&format!("        \\centering\n"));
                subfigure.push_str(&format!("        \\begin{{tikzpicture}}[scale={:.3}]\n", tikz_scale));
                subfigure.push_str(&format!("{}\n", plot.to_tikz(&colors)));
                subfigure.push_str(&format!("        \\end{{tikzpicture}}\n"));
                if let Some(name) = plot.name.clone() {
                    subfigure.push_str(&format!("        \\caption*{{{}}}\n", name));
                }
                subfigure.push_str(&format!("    \\end{{subfigure}}"));
                subfigure
            }).collect::<Vec<String>>().join("\n    ~\n")
        ).collect::<Vec<String>>().join("\n    \\par\\bigskip\n");

        tikz.push_str(&format!("\\begin{{figure}}[ht]\n"));
        tikz.push_str(&format!("    \\centering\n"));
        tikz.push_str(&format!("{}\n", rows));
        tikz.push_str(&format!("\\end{{figure}}\n"));
        tikz
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

    pub fn save_tikz(&self, filename: &String) {

        // Create a path to the desired file.
        let path_str = format!("plots/{}.tikz", filename.clone());
        let path = Path::new(&path_str);
        let display = path.display();

        // Open file in write-only mode, returns `io::Result<File>`.
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why.description()),
            Ok(file) => file
        };

        // Generate TIKZ.
        let tikz = self.to_tikz();

        // Write string to `file`, returns `io::Result<()>`.
        match file.write_all(tikz.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
            Ok(_) => return
        }
    }
}
