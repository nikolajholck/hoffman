use std::cmp::min;

use super::*;

pub type Cube = [[[Point3D; N]; N]; N];
pub const M: usize = 3;
pub type Coord = [usize; M];
pub type Shape = [usize; M];
pub type Kernel = [[[Point3D; KERNEL_DIM]; KERNEL_DIM]; KERNEL_DIM];

pub fn make_coords(shape: Shape) -> Vec<Coord> {
    let axes: Vec<Vec<usize>> = shape.iter().map(|&size| {
        (0..size).collect()
    }).collect();
    product(&axes).iter().map(|list| {
        let mut coord = [0; M];
        for i in 0..M {
            coord[i] = list[i];
        }
        coord
    }).collect()
}

pub fn plot(positions: &Cube, sizes: &Cube, brick: &[IntType; N], name: &String) {
    let dim_labels = ["x", "y", "z"];
    let dims = (0..M).collect::<Vec<usize>>();
    let mut plots = Vec::new();
    for dim in 0..M {
        for level in 0..N {
            let mut rects = Vec::new();
            for i in 0..N {
                for j in 0..N {
                    let mut index = vec!(i, j);
                    index.insert(dim, level);
                    let square_dims = list_except(&dims, &[dim]);
                    let position = positions[index[0]][index[1]][index[2]];
                    let size = sizes[index[0]][index[1]][index[2]];
                    let rectangle = plot::Rectangle {
                        x: position[square_dims[0]], y: position[square_dims[1]],
                        width: size[square_dims[0]], height: size[square_dims[1]]
                    };
                    rects.push(rectangle);
                }
            }
            let square_name = list_except(&dim_labels, &[dim_labels[dim]]).join("");
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
        brick: brick.to_vec(),
        rows: 3,
        columns: N
    };
    figure.save(&format!("cubes/{}", name));
}

pub fn symmetries(cube: &Cube) -> Vec<Cube> {
    let mut symmetries = Vec::new();
    let dims = (0..N).collect::<Vec<usize>>();
    let directions = [Direction::Positive, Direction::Negative];
    let direction_choices = combinations_with_repetition(&directions, N);
    let axis_permutations = permutations(&dims, N);
    for axes in axis_permutations.iter() {
        for directions in direction_choices.iter() {
            let mut symmetry = [[[Point3D { x: 0, y: 0, z: 0 }; N]; N]; N];
            for x in 0..N {
                for y in 0..N {
                    for z in 0..N {
                        let index = [x, y, z];
                        let transform = directions.iter().zip(axes.iter()).map(|(d, a)| {
                            match *d {
                                Direction::Positive => index[*a],
                                Direction::Negative => N - 1 - index[*a]
                            }
                        }).collect::<Vec<usize>>();
                        let size = cube[x][y][z];
                        let new_size = Point3D {
                            x: size[axes[0]],
                            y: size[axes[1]],
                            z: size[axes[2]],
                        };
                        symmetry[transform[0]][transform[1]][transform[2]] = new_size;
                    }
                }
            }
            symmetries.push(symmetry);
        }
    }
    assert!(symmetries.len() == 48, "invalid number of symmetries.");
    symmetries
}

pub fn kernel_is_brick_valid(sizes: &Kernel, coord: &Coord) -> bool {
    let brick = sizes[coord[0]][coord[1]][coord[2]];
    for (dim, &c) in coord.iter().enumerate() {
        let mut index = coord.clone();
        for j in 0..c {
            index[dim] = j;
            if sizes[index[0]][index[1]][index[2]][dim] == brick[dim] {
                return false
            }
        }
    }
    true
}

pub fn position_brick(positions: &mut Cube, &sizes: &Cube, coord: &Coord) {
    let mut pos = Point3D::ZERO;
    for (dim, &c) in coord.iter().enumerate() {
        if c > 0 {
            let mut index = coord.clone();
            index[dim] -= 1;
            pos[dim] = positions[index[0]][index[1]][index[2]][dim] + sizes[index[0]][index[1]][index[2]][dim]
        }
    }
    positions[coord[0]][coord[1]][coord[2]] = pos;
}

pub fn is_brick_valid(positions: &Cube, sizes: &Cube, coord: &Coord) -> bool {
    let brick = sizes[coord[0]][coord[1]][coord[2]];
    for (dim, &c) in coord.iter().enumerate() {
        let mut index = coord.clone();
        for j in 0..c {
            index[dim] = j;
            if sizes[index[0]][index[1]][index[2]][dim] == brick[dim] {
                return false
            }
        }
    }
    !does_intersect(&positions, &sizes, &coord)
}

pub fn does_intersect(positions: &Cube, sizes: &Cube, coord: &Coord) -> bool {
    let (x, y, z) = (coord[0], coord[1], coord[2]);
    let this_intervals = Point3D::make_intervals(&positions[x][y][z], &sizes[x][y][z]);

    let other_x_b = if x == 0 { 0 } else { x - 1 };
    let other_y_b = if y == 0 { 0 } else { y - 1 };
    let other_z_b = if z == 0 { 0 } else { z - 1 };
    let other_x_e = min(x + 2, N);
    let other_y_e = min(y + 2, N);
    let other_z_e = min(z + 2, N);

    for other_x in other_x_b..other_x_e {
        for other_y in other_y_b..other_y_e {
            for other_z in other_z_b..other_z_e {
                if other_x == x && other_y == y && other_z == z { continue } // Skip itself.
                let empty = Point3D { x: 0, y: 0, z: 0};
                if positions[other_x][other_y][other_z] == empty { continue } // Skip empty.
                let other_intervals = Point3D::make_intervals(&positions[other_x][other_y][other_z], &sizes[other_x][other_y][other_z]);
                if other_intervals.iter().zip(this_intervals.iter()).all(|(a, b)| a.intersects(&b)) {
                    return true
                }
            }
        }
    }
    false
}
