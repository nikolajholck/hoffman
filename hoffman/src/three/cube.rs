use std::cmp::min;

use super::*;

pub type Cube = [[[Point3D; N]; N]; N];

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

pub fn makes_sharp_corner(positions: &Cube, sizes: &Cube, coord: [usize; N], comparator: &Comparator) -> bool {
    let this_intervals = Point3D::make_intervals(&positions[coord[0]][coord[1]][coord[2]], &sizes[coord[0]][coord[1]][coord[2]]);
    for i in 0..N {
        if coord[i] > 0 {
            let mut foundation_coord = coord.clone();
            foundation_coord[i] -= 1;
            let foundation_position = positions[foundation_coord[0]][foundation_coord[1]][foundation_coord[2]];
            let foundation_size = sizes[foundation_coord[0]][foundation_coord[1]][foundation_coord[2]];
            if foundation_size == Point3D::ZERO {
                continue;
            }
            let foundation_intervals = Point3D::make_intervals(&foundation_position, &foundation_size);
            let other_dims = (0..N).filter(|&v| v != i).collect::<Vec<_>>();
            for &dim in other_dims.iter() {
                if coord[dim] + 1 < N {
                    let mut other_coord = foundation_coord.clone();
                    other_coord[dim] += 1;
                    let other_position = positions[other_coord[0]][other_coord[1]][other_coord[2]];
                    let other_size = sizes[other_coord[0]][other_coord[1]][other_coord[2]];
                    if other_size == Point3D::ZERO {
                        continue;
                    }
                    let other_intervals = Point3D::make_intervals(&other_position, &other_size);
                    let first = comparator.compare(foundation_intervals[dim].end, this_intervals[dim].end);
                    let second = comparator.compare(foundation_intervals[i].end, other_intervals[i].end);
                    if (first == None || first == Some(Ordering::Greater)) && (second == None || second == Some(Ordering::Greater)) {
                        return true
                    }
                }
            }
        }
    }
    false
}

pub fn position_brick(positions: &mut Cube, &sizes: &Cube, (x, y, z): (usize, usize, usize)) {
    let x_pos = if x == 0 { 0 } else {
        positions[x - 1][y][z].x + sizes[x - 1][y][z].x
    };
    let y_pos = if y == 0 { 0 } else {
        positions[x][y - 1][z].y + sizes[x][y - 1][z].y
    };
    let z_pos = if z == 0 { 0 } else {
        positions[x][y][z - 1].z + sizes[x][y][z - 1].z
    };
    positions[x][y][z] = Point3D { x: x_pos, y: y_pos, z: z_pos };
}

pub fn is_brick_valid(positions: &Cube, sizes: &Cube, (x, y, z): (usize, usize, usize), comparator: &Comparator) -> bool {
    let brick = sizes[x][y][z];
    for i in 0..z {
        if sizes[x][y][i].z == brick.z {
            return false
        }
    }
    for i in 0..y {
        if sizes[x][i][z].y == brick.y {
            return false
        }
    }
    for i in 0..x {
        if sizes[i][y][z].x == brick.x {
            return false
        }
    }
    !does_intersect(&positions, &sizes, (x, y, z), &comparator)
}

pub fn does_intersect(positions: &Cube, sizes: &Cube, (x, y, z): (usize, usize, usize), comparator: &Comparator) -> bool {
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
                if other_intervals.iter().zip(this_intervals.iter()).all(|(a, b)| comparator.intervals_intersect(&a, &b)) {
                    return true
                }
            }
        }
    }
    false
}
