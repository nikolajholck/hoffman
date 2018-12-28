use std::collections::HashMap;
use std::error::Error;

use super::*;
use combinatorics::*;

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
    pub dimension_tuple: Vec<IntType>,
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
    pub fn save_svg(&self, directory: &String, file_name: &String) {
        match utils::write_file(&self.to_svg(), &format!("plots/{}", directory), &format!("{}.svg", file_name)) {
            Err(why) => panic!("Error saving svg: {}", why.description()),
            Ok(_) => return
        }
    }

    pub fn save_tikz(&self, directory: &String, file_name: &String) {
        match utils::write_file(&self.to_tikz(), &format!("plots/{}", directory), &format!("{}.tikz", file_name)) {
            Err(why) => panic!("Error saving tikz: {}", why.description()),
            Ok(_) => return
        }
    }

    fn to_svg(&self) -> String {
        assert!(self.plots.len() == self.rows * self.columns, "Number of plots doesn't match number of rows and columns.");

        let mut colors: HashMap<IntType, usize> = HashMap::new();
        for (i, sum) in combinations(&self.dimension_tuple, 2).iter().map(|li| li.iter().sum()).enumerate() {
            colors.insert(sum, i);
        }

        let mut svg = String::new();
        let dimension_tuple_sum = self.dimension_tuple.iter().sum::<IntType>() as f64;
        let plot_size = (WIDTH - MARGIN * ((self.columns + 1) as f64)) / self.columns as f64;
        let plot_scale = plot_size / dimension_tuple_sum;
        let figure_height = MARGIN * (self.rows + 1) as f64 + plot_size * self.rows as f64;

        svg.push_str(&format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n", WIDTH, figure_height, WIDTH, figure_height));
        for row in 0..self.rows {
            for column in 0..self.columns {
                let x = (MARGIN + plot_size) * column as f64 + MARGIN;
                let y = (MARGIN + plot_size) * row as f64 + MARGIN;
                let plot = &self.plots[row * self.columns + column];
                let name = plot.name.clone().unwrap_or(String::from(""));
                svg.push_str(&format!("<g transform=\"matrix({} 0 0 {} {} {})\">\n", plot_scale, plot_scale, x, y));
                svg.push_str(&format!("<rect x=\"0\" y=\"0\" width=\"{}\" height=\"{}\" style=\"fill:none;stroke:#000;stroke-width:1;\" vector-effect=\"non-scaling-stroke\" />", dimension_tuple_sum, dimension_tuple_sum));
                svg.push_str(&format!("{}\n", plot.to_svg(&colors)));
                svg.push_str(&format!("</g>\n"));
                svg.push_str(&format!("<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-size=\"16\">{}</text>", x + plot_size * 0.5, y + plot_size + 18.0, name));
            }
        }
        svg.push_str("</svg>");
        svg
    }

    fn to_tikz(&self) -> String {
        assert!(self.plots.len() == self.rows * self.columns, "Number of plots doesn't match number of rows and columns.");

        let mut colors: HashMap<IntType, usize> = HashMap::new();
        for (i, sum) in combinations(&self.dimension_tuple, 2).iter().map(|li| li.iter().sum()).enumerate() {
            colors.insert(sum, i);
        }

        let mut tikz = String::new();
        let dimension_tuple_sum: IntType = self.dimension_tuple.iter().sum();
        let figure_scale = 1.0 / (self.columns as f64) - 0.03;
        let text_width = 12.0;
        let tikz_scale = figure_scale * text_width / dimension_tuple_sum as f64;

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
}

pub fn plot_3d(recipe: &Recipe, dimension_tuple: &DimensionTuple, name: &String) {
    const N: usize = 3;
    let mut recipe_builder = RecipeBuilder::new(N, N, vec!(dimension_tuple.clone()));
    recipe_builder.produce(recipe);

    let dim_labels = ["x", "y", "z"];
    let mut plots = Vec::new();
    for dim in 0..N {
        for level in 0..N {
            let rects = recipe_builder.get_rectangles_at(vec!((dim, level)));
            let square_name = utils::list_except(&dim_labels, &[dim_labels[dim]]).join("");
            let plot_name = format!("{}-square at {}={}", square_name, dim_labels[dim], level);
            let plot = plot::Plot {
                name: Some(plot_name),
                rectangles: rects
            };
            plots.push(plot);
        }
    }
    let figure = plot::Figure {
        name: None,
        plots: plots,
        dimension_tuple: dimension_tuple.clone(),
        rows: N,
        columns: N
    };
    figure.save_svg(&String::from("cubes"), name);
    figure.save_tikz(&String::from("cubes"), name);
}

pub fn plot_4d(recipe: &Recipe, dimension_tuple: &DimensionTuple, name: &String) {
    const N: usize = 4;
    const M: usize = 4;
    let mut recipe_builder = RecipeBuilder::new(N, M, vec!(dimension_tuple.clone()));
    recipe_builder.produce(recipe);

    let dim_labels = ["x", "y", "z", "w"];
    let dims: Vec<usize> = (0..M).collect();
    let fixed_dims = combinatorics::combinations(&dims, M - 2);
    let mut plots = Vec::new();
    for fixed in &fixed_dims {
        for level0 in 0..N {
            for level1 in 0..N {
                let rects = recipe_builder.get_rectangles_at(vec!(
                    (fixed[0], level0),
                    (fixed[1], level1)
                ));
                let plot_name = format!("${} = {}$ and ${} = {}$.", dim_labels[fixed[0]], level0 + 1, dim_labels[fixed[1]], level1 + 1);
                let plot = plot::Plot {
                    name: Some(plot_name),
                    rectangles: rects
                };
                plots.push(plot);
            }
        }
    }
    let figure = plot::Figure {
        name: None,
        plots: plots,
        dimension_tuple: dimension_tuple.clone(),
        rows: 24,
        columns: N
    };
    figure.save_svg(&String::from("tesseracts"), name);
    figure.save_tikz(&String::from("tesseracts"), name);
}

pub fn plot_4d_cube(recipe: &Recipe, dimension_tuple: &DimensionTuple, name: &String) {
    const N: usize = 4;
    const M: usize = 3;
    let mut recipe_builder = RecipeBuilder::new(N, M, vec!(dimension_tuple.clone()));
    recipe_builder.produce(recipe);

    let dim_labels = ["x", "y", "z"];
    let dims = (0..M).collect::<Vec<usize>>();
    let fixed_dims = combinatorics::combinations(&dims, M - 2);
    let mut plots = Vec::new();

    for fixed in &fixed_dims {
        let dim = fixed[0];
        for level in 0..N {
            let rects = recipe_builder.get_rectangles_at(vec!(
                (dim, level)
            ));

            let square_name = utils::list_except(&dim_labels, &[dim_labels[dim]]).join("");
            let plot_name = format!("{}-square at {}={}", square_name, dim_labels[dim], level + 1);

            let plot = plot::Plot {
                name: Some(plot_name),
                rectangles: rects
            };
            plots.push(plot);
        }
    }
    let figure = plot::Figure {
        name: None,
        plots: plots,
        dimension_tuple: dimension_tuple.clone(),
        rows: 3,
        columns: N
    };
    figure.save_svg(&String::from("cubes"), name);
    figure.save_tikz(&String::from("cubes"), name);
}
