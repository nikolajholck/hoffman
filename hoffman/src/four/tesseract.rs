use std::cmp::min;

use super::*;

pub type Tesseract = [[[[Point4D; N]; N]; N]; N];
pub type Coord = [usize; N];
pub type Shape = [usize; N];

pub fn make_coords(shape: Shape) -> Vec<Coord> {
    let axes: Vec<Vec<usize>> = shape.iter().map(|&size| {
        (0..size).collect()
    }).collect();
    product(&axes).iter().map(|list| {
        let mut coord = [0; N];
        for i in 0..N {
            coord[i] = list[i];
        }
        coord
    }).collect()
}

pub fn export(positions: &Tesseract, sizes: &Tesseract, brick: &[IntType; N], name: &String) {
    let coords = make_coords([N; N]);
    let mut bricks: Vec<export::Brick> = Vec::new();
    for coord in coords.iter() {
        let position = positions[coord[0]][coord[1]][coord[2]][coord[3]];
        let size = sizes[coord[0]][coord[1]][coord[2]][coord[3]];
        let brick = export::Brick {
            coord: coord.to_vec(),
            position: vec!(position[0], position[1], position[2], position[3]),
            size: vec!(size[0], size[1], size[2], size[3])
        };
        bricks.push(brick);
    }
    let export = export::Export {
        name: Some(format!("{}", name)),
        dimensions: N,
        brick: brick.to_vec(),
        bricks: bricks
    };
    export.save(&format!("tesseracts/{}", name));
}

pub fn plot(positions: &Tesseract, sizes: &Tesseract, brick: &[IntType; N], name: &String) {
    let dim_labels = ["x", "y", "z", "w"];
    let dims: Vec<usize> = (0..N).collect();
    let fixed_dims = combinations(&dims, 2);
    let mut plots = Vec::new();
    for fixed in fixed_dims.iter() {
        for level0 in 0..N {
            for level1 in 0..N {
                let mut rects = Vec::new();
                for i in 0..N {
                    for j in 0..N {
                        let mut index = vec!(i, j);
                        index.insert(fixed[0], level0);
                        index.insert(fixed[1], level1);
                        let square_dims = list_except(&dims, &fixed);
                        let position = positions[index[0]][index[1]][index[2]][index[3]];
                        let size = sizes[index[0]][index[1]][index[2]][index[3]];
                        let rectangle = plot::Rectangle {
                            x: position[square_dims[0]], y: position[square_dims[1]],
                            width: size[square_dims[0]], height: size[square_dims[1]]
                        };
                        rects.push(rectangle);
                    }
                }
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
        brick: brick.to_vec(),
        rows: 24,
        columns: N
    };
    figure.save(&format!("tesseracts/{}", name));
    figure.save_tikz(&format!("tesseracts/{}", name));
}

pub fn makes_sharp_corner(positions: &Tesseract, sizes: &Tesseract, coord: &Coord) -> bool {
    let this_intervals = Point4D::make_intervals(&positions[coord[0]][coord[1]][coord[2]][coord[3]], &sizes[coord[0]][coord[1]][coord[2]][coord[3]]);
    let directions: Vec<usize> = coord.iter().enumerate().filter(|&(_, &c)| c > 0).map(|(i, _)| i).collect();
    for &i in &directions {
        let mut foundation_coord = coord.clone();
        foundation_coord[i] -= 1;
        let foundation_position = positions[foundation_coord[0]][foundation_coord[1]][foundation_coord[2]][foundation_coord[3]];
        let foundation_size = sizes[foundation_coord[0]][foundation_coord[1]][foundation_coord[2]][foundation_coord[3]];
        if foundation_size == Point4D::ZERO { continue }
        let foundation_intervals = Point4D::make_intervals(&foundation_position, &foundation_size);
        let other_dims = (0..N).filter(|&v| v != i).collect::<Vec<_>>();
        for &dim in other_dims.iter() {
            if coord[dim] + 1 < N {
                let mut other_coord = coord.clone();
                other_coord[i] -= 1;
                other_coord[dim] += 1;
                let other_position = positions[other_coord[0]][other_coord[1]][other_coord[2]][other_coord[3]];
                let other_size = sizes[other_coord[0]][other_coord[1]][other_coord[2]][other_coord[3]];
                if other_size == Point4D::ZERO { continue }
                let other_intervals = Point4D::make_intervals(&other_position, &other_size);
                if foundation_intervals[dim].end > this_intervals[dim].end
                && foundation_intervals[i].end > other_intervals[i].end {
                    return true
                }
            }
        }
    }
    if directions.len() >= 2 {
        let combs = combinations(&directions, 2);
        for comb in &combs {
            let mut foundation_coord = coord.clone();
            foundation_coord[comb[0]] -= 1;
            foundation_coord[comb[1]] -= 1;
            let foundation_position = positions[foundation_coord[0]][foundation_coord[1]][foundation_coord[2]][foundation_coord[3]];
            let foundation_size = sizes[foundation_coord[0]][foundation_coord[1]][foundation_coord[2]][foundation_coord[3]];
            if foundation_size == Point4D::ZERO { continue }
            let foundation_intervals = Point4D::make_intervals(&foundation_position, &foundation_size);
            let other_dims = list_except(&(0..N).collect::<Vec<_>>(), &comb);
            for &dim in &other_dims {
                if coord[dim] + 1 < N {
                    let mut a_other_coord = coord.clone();
                    a_other_coord[comb[0]] -= 1;
                    a_other_coord[dim] += 1;
                    let a_other_position = positions[a_other_coord[0]][a_other_coord[1]][a_other_coord[2]][a_other_coord[3]];
                    let a_other_size = sizes[a_other_coord[0]][a_other_coord[1]][a_other_coord[2]][a_other_coord[3]];
                    if a_other_size == Point4D::ZERO { continue }

                    let mut b_other_coord = coord.clone();
                    b_other_coord[comb[1]] -= 1;
                    b_other_coord[dim] += 1;
                    let b_other_position = positions[b_other_coord[0]][b_other_coord[1]][b_other_coord[2]][b_other_coord[3]];
                    let b_other_size = sizes[b_other_coord[0]][b_other_coord[1]][b_other_coord[2]][b_other_coord[3]];
                    if b_other_size == Point4D::ZERO { continue }

                    let a_other_intervals = Point4D::make_intervals(&a_other_position, &a_other_size);
                    let b_other_intervals = Point4D::make_intervals(&b_other_position, &b_other_size);

                    if foundation_intervals[dim].end > this_intervals[dim].end
                    && foundation_intervals[comb[0]].end > a_other_intervals[comb[0]].end
                    && foundation_intervals[comb[1]].end > b_other_intervals[comb[1]].end {
                        //println!("3D Sharp");
                        return true
                    }
                }
            }
        }
    }
    if directions.len() >= 3 {
        let combs = combinations(&directions, 3);
        for comb in &combs {
            let mut foundation_coord = coord.clone();
            foundation_coord[comb[0]] -= 1;
            foundation_coord[comb[1]] -= 1;
            foundation_coord[comb[2]] -= 1;
            let foundation_position = positions[foundation_coord[0]][foundation_coord[1]][foundation_coord[2]][foundation_coord[3]];
            let foundation_size = sizes[foundation_coord[0]][foundation_coord[1]][foundation_coord[2]][foundation_coord[3]];
            if foundation_size == Point4D::ZERO { continue }
            let foundation_intervals = Point4D::make_intervals(&foundation_position, &foundation_size);
            let other_dims = list_except(&(0..N).collect::<Vec<_>>(), &comb);
            //println!("4 other len: {:?}", other_dims.len());
            for &dim in &other_dims {
                if coord[dim] + 1 < N {
                    let mut a_other_coord = coord.clone();
                    a_other_coord[comb[0]] -= 1;
                    a_other_coord[dim] += 1;
                    let a_other_position = positions[a_other_coord[0]][a_other_coord[1]][a_other_coord[2]][a_other_coord[3]];
                    let a_other_size = sizes[a_other_coord[0]][a_other_coord[1]][a_other_coord[2]][a_other_coord[3]];
                    if a_other_size == Point4D::ZERO { continue }

                    let mut b_other_coord = coord.clone();
                    b_other_coord[comb[1]] -= 1;
                    b_other_coord[dim] += 1;
                    let b_other_position = positions[b_other_coord[0]][b_other_coord[1]][b_other_coord[2]][b_other_coord[3]];
                    let b_other_size = sizes[b_other_coord[0]][b_other_coord[1]][b_other_coord[2]][b_other_coord[3]];
                    if b_other_size == Point4D::ZERO { continue }

                    let mut c_other_coord = coord.clone();
                    c_other_coord[comb[2]] -= 1;
                    c_other_coord[dim] += 1;
                    let c_other_position = positions[c_other_coord[0]][c_other_coord[1]][c_other_coord[2]][c_other_coord[3]];
                    let c_other_size = sizes[c_other_coord[0]][c_other_coord[1]][c_other_coord[2]][c_other_coord[3]];
                    if c_other_size == Point4D::ZERO { continue }

                    let a_other_intervals = Point4D::make_intervals(&a_other_position, &a_other_size);
                    let b_other_intervals = Point4D::make_intervals(&b_other_position, &b_other_size);
                    let c_other_intervals = Point4D::make_intervals(&c_other_position, &c_other_size);

                    if foundation_intervals[dim].end > this_intervals[dim].end
                    && foundation_intervals[comb[0]].end > a_other_intervals[comb[0]].end
                    && foundation_intervals[comb[1]].end > b_other_intervals[comb[1]].end
                    && foundation_intervals[comb[2]].end > c_other_intervals[comb[2]].end {
                        println!("4D Sharp");
                        return true
                    }
                }
            }
        }
    }
    false
}

pub fn position_brick(positions: &mut Tesseract, sizes: &Tesseract, coord: &Coord) {
    let mut pos = Point4D::ZERO;
    for (dim, &c) in coord.iter().enumerate() {
        if c > 0 {
            let mut index = coord.clone();
            index[dim] -= 1;
            pos[dim] = positions[index[0]][index[1]][index[2]][index[3]][dim] + sizes[index[0]][index[1]][index[2]][index[3]][dim]
        }
    }
    positions[coord[0]][coord[1]][coord[2]][coord[3]] = pos;
}

pub fn is_brick_valid(positions: &Tesseract, sizes: &Tesseract, coord: &Coord) -> bool {
    let brick = sizes[coord[0]][coord[1]][coord[2]][coord[3]];
    for (dim, &c) in coord.iter().enumerate() {
        let mut index = coord.clone();
        for j in 0..c {
            index[dim] = j;
            if sizes[index[0]][index[1]][index[2]][index[3]][dim] == brick[dim] {
                return false
            }
        }
    }
    !does_intersect(&positions, &sizes, &coord)
}

pub fn does_intersect(positions: &Tesseract, sizes: &Tesseract, coord: &Coord) -> bool {
    let (x, y, z, w) = (coord[0], coord[1], coord[2], coord[3]);
    let this_intervals = Point4D::make_intervals(&positions[x][y][z][w], &sizes[x][y][z][w]);

    let other_x_b = if x == 0 { 0 } else { x - 1 };
    let other_y_b = if y == 0 { 0 } else { y - 1 };
    let other_z_b = if z == 0 { 0 } else { z - 1 };
    let other_w_b = if w == 0 { 0 } else { w - 1 };
    let other_x_e = min(x + 2, N);
    let other_y_e = min(y + 2, N);
    let other_z_e = min(z + 2, N);
    let other_w_e = min(w + 2, N);

    for other_x in other_x_b..other_x_e {
        for other_y in other_y_b..other_y_e {
            for other_z in other_z_b..other_z_e {
                for other_w in other_w_b..other_w_e {
                    if other_x == x && other_y == y && other_z == z && other_w == w { continue } // Skip itself.
                    let empty = Point4D { x: 0, y: 0, z: 0, w: 0};
                    if positions[other_x][other_y][other_z][other_w] == empty { continue } // Skip empty.
                    let other_intervals = Point4D::make_intervals(&positions[other_x][other_y][other_z][other_w], &sizes[other_x][other_y][other_z][other_w]);
                    if other_intervals.iter().zip(this_intervals.iter()).all(|(a, b)| a.intersects(&b)) {
                        return true
                    }
                }
            }
        }
    }
    false
}
