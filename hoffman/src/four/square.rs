use std::cmp::min;

use super::*;

pub type Square = [[Point2D; N]; N];
pub const M: usize = 2;
pub type Coord = [usize; M];
pub type Shape = [usize; M];
pub type Kernel = [[Point2D; KERNEL_DIM]; KERNEL_DIM];

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

pub fn kernel_plot(sizes: &Kernel, brick: &[IntType; N], name: &String) {
    let mut plots = Vec::new();
    let center: IntType = brick.iter().sum::<IntType>() / 2;

    for kernel in kernel_symmetries(sizes).iter() {
        let mut rects = Vec::new();
        for x in 0..KERNEL_DIM {
            for y in 0..KERNEL_DIM {
                let size = kernel[x][y];
                let position = Point2D {
                    x: center + if x == 0 { -size.x } else { 0 },
                    y: center + if y == 0 { -size.y } else { 0 }
                };
                let rectangle = plot::Rectangle {
                    x: position.x, y: position.y,
                    width: size.x, height: size.y
                };
                rects.push(rectangle);
            }
        }
        let plot = plot::Plot {
            name: None,
            rectangles: rects
        };
        plots.push(plot);
    }
    let figure = plot::Figure {
        name: None,
        plots: plots,
        brick: brick.to_vec(),
        rows: 2,
        columns: 4
    };
    figure.save(&format!("squares/kernels/{}", name));
}

pub fn kernel_drain_symmetries(kernels: &mut Vec<Kernel>) {
    let mut i: usize = 0;
    while i < kernels.len() {
        let kernel = kernels[i];
        let symmetries = kernel_symmetries(&kernel);
        let mut deleted_count: usize = 0;
        for j in (i + 1..kernels.len()).rev() { // Check subsequent packings
            let suspect_kernel = kernels[j];
            if symmetries.contains(&suspect_kernel) {
                kernels.remove(j); // Remove duplicate.
                deleted_count += 1;
            }
        }
        if deleted_count != 8 - 1 {
            //println!("Special kernel (deleted: {:?})", deleted_count);
        }
        i += 1;
    }
}

pub fn kernel_symmetries(kernel: &Kernel) -> Vec<Kernel> {
    let mut symmetries = Vec::new();
    let dims = (0..M).collect::<Vec<usize>>();
    let directions = [Direction::Positive, Direction::Negative];
    let direction_choices = combinations_with_repetition(&directions, M);
    let axis_permutations = permutations(&dims, M);
    for axes in axis_permutations.iter() {
        for directions in direction_choices.iter() {
            let mut symmetry = [[Point2D { x: 0, y: 0 }; KERNEL_DIM]; KERNEL_DIM];
            for x in 0..KERNEL_DIM {
                for y in 0..KERNEL_DIM {
                    let index = [x, y];
                    let transform = directions.iter().zip(axes.iter()).map(|(d, a)| {
                        match *d {
                            Direction::Positive => index[*a],
                            Direction::Negative => KERNEL_DIM - 1 - index[*a]
                        }
                    }).collect::<Vec<usize>>();
                    let size = kernel[x][y];
                    let new_size = Point2D {
                        x: size[axes[0]],
                        y: size[axes[1]]
                    };
                    symmetry[transform[0]][transform[1]] = new_size;
                }
            }
            symmetries.push(symmetry);
        }
    }
    assert!(symmetries.len() == 8, "invalid number of symmetries.");
    symmetries
}

pub fn kernel_is_self_symmetric(kernel: &Kernel) -> bool {
    kernel_symmetries(kernel)[1..].contains(kernel)
}

pub fn kernel_is_brick_valid(sizes: &Kernel, coord: &Coord) -> bool {
    let brick = sizes[coord[0]][coord[1]];
    for (dim, &c) in coord.iter().enumerate() {
        let mut index = coord.clone();
        for j in 0..c {
            index[dim] = j;
            if sizes[index[0]][index[1]][dim] == brick[dim] {
                return false
            }
        }
    }
    true
}

pub fn plot(positions: &Square, sizes: &Square, brick: &[IntType], name: &String) {
    let mut rects = Vec::new();

    for x in 0..N {
        for y in 0..N {
            let position = positions[x][y];
            let size = sizes[x][y];
            let rectangle = plot::Rectangle {
                x: position.x, y: position.y,
                width: size.x, height: size.y
            };
            rects.push(rectangle);
        }
    }

    let plot = plot::Plot {
        name: None,
        rectangles: rects
    };
    let plots = vec!(plot);

    let figure = plot::Figure {
        name: None,
        plots: plots,
        brick: brick.to_vec(),
        rows: 1,
        columns: 1
    };
    figure.save(&format!("squares/{}", name));
}

pub fn drain_symmetries(squares: &mut Vec<Square>) {
    let mut i: usize = 0;
    while i < squares.len() {
        let square = squares[i];
        let symmetries = symmetries(&square);
        let mut deleted_count: usize = 0;
        for j in (i + 1..squares.len()).rev() { // Check subsequent packings
            let suspect_square = squares[j];
            if symmetries.contains(&suspect_square) {
                squares.remove(j); // Remove duplicate.
                deleted_count += 1;
            }
        }
        if deleted_count != 8 - 1 {
            //println!("Special square (deleted: {:?})", deleted_count);
        }
        i += 1;
    }
}

pub fn symmetries(square: &Square) -> Vec<Square> {
    let mut symmetries = Vec::new();
    let dims = (0..M).collect::<Vec<usize>>();
    let directions = [Direction::Positive, Direction::Negative];
    let direction_choices = combinations_with_repetition(&directions, M);
    let axis_permutations = permutations(&dims, M);
    for axes in axis_permutations.iter() {
        for directions in direction_choices.iter() {
            let mut symmetry = [[Point2D { x: 0, y: 0 }; N]; N];
            for x in 0..N {
                for y in 0..N {
                    let index = [x, y];
                    let transform = directions.iter().zip(axes.iter()).map(|(d, a)| {
                        match *d {
                            Direction::Positive => index[*a],
                            Direction::Negative => N - 1 - index[*a]
                        }
                    }).collect::<Vec<usize>>();
                    let size = square[x][y];
                    let new_size = Point2D {
                        x: size[axes[0]],
                        y: size[axes[1]]
                    };
                    symmetry[transform[0]][transform[1]] = new_size;
                }
            }
            symmetries.push(symmetry);
        }
    }
    assert!(symmetries.len() == 8, "invalid number of symmetries.");
    symmetries
}

pub fn position_brick(positions: &mut Square, &sizes: &Square, coord: &Coord) {
    let mut pos = Point2D::ZERO;
    for (dim, &c) in coord.iter().enumerate() {
        if c > 0 {
            let mut index = coord.clone();
            index[dim] -= 1;
            pos[dim] = positions[index[0]][index[1]][dim] + sizes[index[0]][index[1]][dim]
        }
    }
    positions[coord[0]][coord[1]] = pos;
}

pub fn is_brick_valid(positions: &Square, sizes: &Square, coord: &Coord) -> bool {
    let brick = sizes[coord[0]][coord[1]];
    for (dim, &c) in coord.iter().enumerate() {
        let mut index = coord.clone();
        for j in 0..c {
            index[dim] = j;
            if sizes[index[0]][index[1]][dim] == brick[dim] {
                return false
            }
        }
    }
    !does_intersect(&positions, &sizes, &coord)
}

pub fn does_intersect(positions: &Square, sizes: &Square, coord: &Coord) -> bool {
    let (x, y) = (coord[0], coord[1]);

    let this_intervals = Point2D::make_intervals(&positions[x][y], &sizes[x][y]);

    let other_x_b = if x == 0 { 0 } else { x - 1 };
    let other_y_b = if y == 0 { 0 } else { y - 1 };
    let other_x_e = min(x + 2, N);
    let other_y_e = min(y + 2, N);

    for other_x in other_x_b..other_x_e {
        for other_y in other_y_b..other_y_e {
            if other_x == x && other_y == y { continue } // Skip itself.
            let other_intervals = Point2D::make_intervals(&positions[other_x][other_y], &sizes[other_x][other_y]);
            if other_intervals.iter().zip(this_intervals.iter()).all(|(a, b)| a.intersects(&b)) {
                return true
            }
        }
    }
    false
}
