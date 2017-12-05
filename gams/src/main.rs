extern crate hoffman;

use hoffman::*;
use std::cmp::{Ordering, min};

pub type Coord = Vec<usize>;

pub fn make_coords(shape: Coord) -> Vec<Coord> {
    let axes: Vec<Vec<usize>> = shape.iter().map(|&size| {
        (1..size + 1).collect()
    }).collect();
    product(&axes)
}

fn main() {
    let n = 4;
    let coords = make_coords(vec!(n; n));
    let mut next_u_index = 1;
    let mut next_pair_index = 1;
    let mut equations = vec!();
    for a in &coords {
        for b in &coords {
            if compare_coords(a, b) == Ordering::Less
            && distance_coords(a, b) == 1
            && !are_too_close(a, b) {
                //println!("{:?}", (a, b));
                let mut logic_terms = vec!();
                for (d, (x, y)) in a.iter().zip(b.iter()).enumerate() {
                    if x != y {
                        let m_index = min(x, y);
                        let terms_count = *min(x, y);
                        let a_str = get_terms(&a, d, terms_count);
                        let b_str = get_terms(&b, d, terms_count);
                        let (ineq, sign) = if x < y {
                            ("=l=", "+")
                        } else {
                            ("=g=", "-")
                        };
                        equations.push(format!("inter_{}.. {} {} {} {} (1 - u('{}')) * M('{}');",
                        next_u_index,
                        a_str, ineq, b_str,
                        sign, next_u_index, m_index));
                        logic_terms.push(format!("u('{}')", next_u_index));
                        next_u_index += 1;
                    }
                }
                equations.push(format!("or_{}.. {} + z('{}') =g= 1;", next_pair_index, logic_terms.join(" + "), next_pair_index));
                next_pair_index += 1;
            }
        }
    }
    println!("Equations");
    println!("{}", (1..next_u_index)
        .map(|u_index| format!("inter_{}", u_index))
        .collect::<Vec<_>>()
        .join("\n")
    );
    println!("{};", (1..next_pair_index)
        .map(|pair_index| format!("or_{}", pair_index))
        .collect::<Vec<_>>()
        .join("\n")
    );
    for eqn in &equations {
        println!("{}", eqn);
    }

    //println!("{:?}: {:?}", n, next_pair_index - 1);
}

fn get_terms(coord: &Coord, dimension: usize, terms_count: usize) -> String {
    let terms: Vec<String> = (1..terms_count+1).map(|i| {
        let mut c = coord.clone();
        c[dimension] = i;
        let inner_coord = c.iter().map(|v| format!("'{}'", v)).collect::<Vec<_>>().join(", ");
        format!("w({}, '{}')", inner_coord, dimension + 1)
    }).collect();
    terms.join(" + ")
}

fn compare_coords(a: &Coord, b: &Coord) -> Ordering {
    assert!(a.len() == b.len());
    for (x, y) in a.iter().zip(b.iter()) {
        if x != y {
            return x.cmp(y)
        }
    }
    assert!(a == b);
    return Ordering::Equal;
}

fn distance_coords(a: &Coord, b: &Coord) -> usize {
    a.iter().zip(b.iter()).map(|(x, y)| {
        if x <= y { y - x } else { x - y }
    }).max().unwrap()
}

fn are_too_close(a: &Coord, b: &Coord) -> bool {
    a.iter().zip(b.iter()).filter(|&(a, b)| a != b).count() <= 1
}
