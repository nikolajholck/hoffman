use std::cmp::min;

use super::*;

pub type Cube = [[[Point3D; N]; N]; N];
pub type Coord = [usize; N];

pub fn make_coords() -> Vec<Coord> {
    let axes: Vec<Vec<usize>> = (0..N).map(|_| {
        (0..N).collect()
    }).collect();
    product(&axes).iter().map(|list| {
        let mut coord = [0; N];
        for i in 0..N {
            coord[i] = list[i];
        }
        coord
    }).collect()
}

pub fn export(positions: &Cube, sizes: &Cube, brick: &[IntType; N], name: &String) {
    let coords = make_coords();
    let mut bricks: Vec<export::Brick> = Vec::new();
    for coord in coords.iter() {
        let position = positions[coord[0]][coord[1]][coord[2]];
        let size = sizes[coord[0]][coord[1]][coord[2]];
        let brick = export::Brick {
            coord: coord.to_vec(),
            position: vec!(position[0], position[1], position[2]),
            size: vec!(size[0], size[1], size[2]),
        };
        bricks.push(brick);
    }
    let export = export::Export {
        name: Some(format!("{}", name)),
        dimensions: N,
        brick: brick.to_vec(),
        bricks: bricks
    };
    export.save(&format!("cubes/{}", name));
}

pub fn plot(positions: &Cube, sizes: &Cube, brick: &[IntType; N], name: &String) {
    let dim_labels = ["x", "y", "z"];
    let dims = (0..N).collect::<Vec<usize>>();
    let mut plots = Vec::new();
    for dim in 0..N {
        for level in 0..N {
            let mut rects = Vec::new();
            for i in 0..N {
                for j in 0..N {
                    let mut index = vec!(i, j);
                    index.insert(dim, level);
                    let plane_dims = list_except(&dims, &[dim]);
                    let position = positions[index[0]][index[1]][index[2]];
                    let size = sizes[index[0]][index[1]][index[2]];
                    let rectangle = plot::Rectangle {
                        x: position[plane_dims[0]], y: position[plane_dims[1]],
                        width: size[plane_dims[0]], height: size[plane_dims[1]]
                    };
                    rects.push(rectangle);
                }
            }
            let plane_name = list_except(&dim_labels, &[dim_labels[dim]]).join("");
            let plot_name = format!("{}-plane at {}={}", plane_name, dim_labels[dim], level);

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
        rows: N,
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
    for axes in &axis_permutations {
        for directions in &direction_choices {
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

pub fn drain_symmetries(packings: &mut Vec<(Cube, Cube)>) {
    let mut i: usize = 0;
    while i < packings.len() {
        let cube = packings[i].1;
        let symmetries = symmetries(&cube);
        let mut deleted_count: usize = 0;
        for j in (i + 1..packings.len()).rev() { // Check subsequent packings
            let suspect_cube = packings[j].1;
            if symmetries.contains(&suspect_cube) {
                packings.remove(j); // Remove duplicate.
                deleted_count += 1;
            }
        }
        if deleted_count != 48 - 1 {
            println!("Special symmetry (deleted: {:?})", deleted_count);
        }
        i += 1;
    }
}

pub fn makes_sharp_corner(positions: &Cube, sizes: &Cube, coord: &[usize; N]) -> bool {
    let this_intervals = Point3D::make_intervals(&positions[coord[0]][coord[1]][coord[2]], &sizes[coord[0]][coord[1]][coord[2]]);
    let directions = coord.iter().enumerate().filter(|&(_, &c)| c > 0).map(|(i, _)| i).collect::<Vec<usize>>();
    for &i in &directions {
        let mut foundation_coord = coord.clone();
        foundation_coord[i] -= 1;
        let foundation_position = positions[foundation_coord[0]][foundation_coord[1]][foundation_coord[2]];
        let foundation_size = sizes[foundation_coord[0]][foundation_coord[1]][foundation_coord[2]];
        if foundation_size == Point3D::ZERO { continue }
        let foundation_intervals = Point3D::make_intervals(&foundation_position, &foundation_size);
        let other_dims = (0..N).filter(|&v| v != i).collect::<Vec<_>>();
        for &dim in &other_dims {
            if coord[dim] + 1 < N {
                let mut other_coord = coord.clone();
                other_coord[i] -= 1;
                other_coord[dim] += 1;
                let other_position = positions[other_coord[0]][other_coord[1]][other_coord[2]];
                let other_size = sizes[other_coord[0]][other_coord[1]][other_coord[2]];
                if other_size == Point3D::ZERO { continue }
                let other_intervals = Point3D::make_intervals(&other_position, &other_size);
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
            let foundation_position = positions[foundation_coord[0]][foundation_coord[1]][foundation_coord[2]];
            let foundation_size = sizes[foundation_coord[0]][foundation_coord[1]][foundation_coord[2]];
            if foundation_size == Point3D::ZERO { continue }
            let foundation_intervals = Point3D::make_intervals(&foundation_position, &foundation_size);
            let other_dims = list_except(&(0..N).collect::<Vec<_>>(), &comb);
            for &dim in &other_dims {
                if coord[dim] + 1 < N {
                    let mut a_other_coord = coord.clone();
                    a_other_coord[comb[0]] -= 1;
                    a_other_coord[dim] += 1;
                    let a_other_position = positions[a_other_coord[0]][a_other_coord[1]][a_other_coord[2]];
                    let a_other_size = sizes[a_other_coord[0]][a_other_coord[1]][a_other_coord[2]];
                    if a_other_size == Point3D::ZERO { continue }

                    let mut b_other_coord = coord.clone();
                    b_other_coord[comb[1]] -= 1;
                    b_other_coord[dim] += 1;
                    let b_other_position = positions[b_other_coord[0]][b_other_coord[1]][b_other_coord[2]];
                    let b_other_size = sizes[b_other_coord[0]][b_other_coord[1]][b_other_coord[2]];
                    if b_other_size == Point3D::ZERO { continue }

                    let a_other_intervals = Point3D::make_intervals(&a_other_position, &a_other_size);
                    let b_other_intervals = Point3D::make_intervals(&b_other_position, &b_other_size);

                    if foundation_intervals[dim].end > this_intervals[dim].end
                    && foundation_intervals[comb[0]].end > a_other_intervals[comb[0]].end
                    && foundation_intervals[comb[1]].end > b_other_intervals[comb[1]].end {
                        return true
                    }
                }
            }
        }
    }
    false
}

pub fn position_brick(positions: &mut Cube, &sizes: &Cube, coord: &[usize; N]) {
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

pub fn is_brick_valid(positions: &Cube, sizes: &Cube, coord: &[usize; N]) -> bool {
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

pub fn does_intersect(positions: &Cube, sizes: &Cube, coord: &[usize; N]) -> bool {
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
                let other_intervals = Point3D::make_intervals(&positions[other_x][other_y][other_z], &sizes[other_x][other_y][other_z]);
                if other_intervals.iter().zip(this_intervals.iter()).all(|(a, b)| a.intersects(&b)) {
                    return true
                }
            }
        }
    }
    false
}
