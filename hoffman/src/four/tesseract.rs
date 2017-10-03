use std::cmp::min;

use super::*;

pub type Tesseract = [[[[Point4D; N]; N]; N]; N];

/*
pub fn plot(positions: &Solid, sizes: &Solid, brick: &[IntType], name: &String) {
    let dim_labels = ["x", "y", "z"];
    let dims = (0..3).collect::<Vec<usize>>();
    let mut plots = Vec::new();
    for dim in 0..3 {
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
        rows: 3,
        columns: N
    };
    figure.save(&format!("solids/{}", name));
}*/


/*
pub fn symmetries(solid: &Solid) -> Vec<Solid> {
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
                        let size = solid[x][y][z];
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
}*/

/*pub fn kernel_drain_symmetries(kernels: &mut Vec<Kernel>) {
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
        if deleted_count != 7 {
            //println!("Special kernel (deleted: {:?})", deleted_count);
        }
        i += 1;
    }
}

pub fn kernel_symmetries(kernel: &Kernel) -> [Kernel; 8] {
    let kernel_clone = kernel_clone(kernel);
    let kernel_flipped = kernel_flip(kernel);
    [
        kernel_clone,
        kernel_rotate(&kernel),
        kernel_rotate(&kernel_rotate(&kernel)),
        kernel_rotate(&kernel_rotate(&kernel_rotate(&kernel))),
        kernel_flipped,
        kernel_rotate(&kernel_flipped),
        kernel_rotate(&kernel_rotate(&kernel_flipped)),
        kernel_rotate(&kernel_rotate(&kernel_rotate(&kernel_flipped)))
    ]
}

pub fn kernel_has_symmetries(kernel: &Kernel) -> bool {
    kernel_symmetries(kernel)[1..].contains(kernel)
}

pub fn kernel_clone(kernel: &Kernel) -> Kernel {
    let mut cloned_kernel = [[Point2D { x: 0, y: 0 }; KERNEL_DIM]; KERNEL_DIM];
    for x in 0..KERNEL_DIM {
        for y in 0..KERNEL_DIM {
            cloned_kernel[x][y] = kernel[x][y];
        }
    }
    cloned_kernel
}

pub fn kernel_flip(kernel: &Kernel) -> Kernel {
    let mut flipped_kernel = [[Point2D { x: 0, y: 0 }; KERNEL_DIM]; KERNEL_DIM];
    for x in 0..KERNEL_DIM {
        for y in 0..KERNEL_DIM {
            flipped_kernel[x][y] = kernel[y][x].flip();
        }
    }
    flipped_kernel
}

pub fn kernel_rotate(kernel: &Kernel) -> Kernel {
    let mut rotated_kernel = [[Point2D { x: 0, y: 0 }; KERNEL_DIM]; KERNEL_DIM];
    for x in 0..KERNEL_DIM {
        for y in 0..KERNEL_DIM {
            rotated_kernel[x][y] = kernel[y][KERNEL_DIM - 1 - x].rotate();
        }
    }
    rotated_kernel
}*/

/*pub fn kernel_is_brick_valid(sizes: &Kernel, (x, y, z): (usize, usize, usize)) -> bool {
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
    true
}*/

/*pub fn drain_symmetries(planes: &mut Vec<Plane>) {
    let mut i: usize = 0;
    while i < planes.len() {
        let plane = planes[i];
        let symmetries = symmetries(&plane);
        let mut deleted_count: usize = 0;
        for j in (i + 1..planes.len()).rev() { // Check subsequent packings
            let suspect_plane = planes[j];
            if symmetries.contains(&suspect_plane) {
                planes.remove(j); // Remove duplicate.
                deleted_count += 1;
            }
        }
        if deleted_count != 7 {
            //println!("Special plane (deleted: {:?})", deleted_count);
        }
        i += 1;
    }
}

pub fn symmetries(plane: &Plane) -> [Plane; 8] {
    let clone = clone(plane);
    let flipped = flip(plane);
    [
        clone,
        rotate(&plane),
        rotate(&rotate(&plane)),
        rotate(&rotate(&rotate(&plane))),
        flipped,
        rotate(&flipped),
        rotate(&rotate(&flipped)),
        rotate(&rotate(&rotate(&flipped)))
    ]
}

}*/

pub fn position_brick(positions: &mut Tesseract, sizes: &Tesseract, (x, y, z, w): (usize, usize, usize, usize)) {
    let x_pos = if x == 0 { 0 } else {
        positions[x - 1][y][z][w].x + sizes[x - 1][y][z][w].x
    };
    let y_pos = if y == 0 { 0 } else {
        positions[x][y - 1][z][w].y + sizes[x][y - 1][z][w].y
    };
    let z_pos = if z == 0 { 0 } else {
        positions[x][y][z - 1][w].z + sizes[x][y][z - 1][w].z
    };
    let w_pos = if w == 0 { 0 } else {
        positions[x][y][z][w - 1].w + sizes[x][y][z][w - 1].w
    };
    positions[x][y][z][w] = Point4D { x: x_pos, y: y_pos, z: z_pos, w: w_pos };
}

pub fn is_brick_valid(positions: &Tesseract, sizes: &Tesseract, (x, y, z, w): (usize, usize, usize, usize), comparator: &Comparator) -> bool {
    let brick = sizes[x][y][z][w];
    for i in 0..N {
        if i != w && sizes[x][y][z][i].w == brick.w {
            return false
        }
    }
    for i in 0..N {
        if i != z && sizes[x][y][i][w].z == brick.z {
            return false
        }
    }
    for i in 0..N {
        if i != y && sizes[x][i][z][w].y == brick.y {
            return false
        }
    }
    for i in 0..N {
        if i != x && sizes[i][y][z][w].x == brick.x {
            return false
        }
    }
    !does_intersect(&positions, &sizes, (x, y, z, w), &comparator)
}

pub fn does_intersect(positions: &Tesseract, sizes: &Tesseract, (x, y, z, w): (usize, usize, usize, usize), comparator: &Comparator) -> bool {
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
                    if other_intervals.iter().zip(this_intervals.iter()).all(|(a, b)| comparator.intervals_intersect(&a, &b)) {
                        return true
                    }
                }
            }
        }
    }
    false
}
