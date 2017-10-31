pub mod three;
pub mod four;
pub mod plot;
pub mod export;

#[cfg(test)]
mod tests;

use std::fmt;
use std::ops::{Index, IndexMut};

pub type IntType = i32;

#[derive(Clone)]
enum Direction {
    Positive,
    Negative
}

#[derive(Clone, Debug, PartialEq)]
pub struct Interval {
    begin: IntType,
    end: IntType
}

impl Interval {
    pub fn is_degenerate(&self) -> bool {
        return self.begin >= self.end
    }

    pub fn is_zero(&self) -> bool {
        self.begin == 0 && self.end == 0
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Point2D {
    pub x: IntType,
    pub y: IntType
}

impl Point2D {
    pub const ZERO: Point2D = Point2D { x: 0, y: 0 };

    pub fn make_intervals(position: &Point2D, size: &Point2D) -> [Interval; 2] {
        [Interval { begin: position.x, end: position.x + size.x },
         Interval { begin: position.y, end: position.y + size.y }]
    }
}

impl Index<usize> for Point2D {
    type Output = IntType;

    fn index(&self, index: usize) -> &IntType {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Point2D index out of bounds.")
        }
    }
}

impl IndexMut<usize> for Point2D {
    fn index_mut(&mut self, index: usize) -> &mut IntType {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Point2D index out of bounds.")
        }
    }
}

impl fmt::Debug for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Point3D {
    pub x: IntType,
    pub y: IntType,
    pub z: IntType
}

impl Point3D {
    pub const ZERO: Point3D = Point3D { x: 0, y: 0, z: 0 };

    pub fn make_intervals(position: &Point3D, size: &Point3D) -> [Interval; 3] {
        [Interval { begin: position.x, end: position.x + size.x },
         Interval { begin: position.y, end: position.y + size.y },
         Interval { begin: position.z, end: position.z + size.z }]
    }
}

impl fmt::Debug for Point3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Index<usize> for Point3D {
    type Output = IntType;

    fn index(&self, index: usize) -> &IntType {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Point3D index out of bounds.")
        }
    }
}

impl IndexMut<usize> for Point3D {
    fn index_mut(&mut self, index: usize) -> &mut IntType {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Point3D index out of bounds.")
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Point4D {
    pub x: IntType,
    pub y: IntType,
    pub z: IntType,
    pub w: IntType
}

impl Point4D {
    pub const ZERO: Point4D = Point4D { x: 0, y: 0, z: 0, w: 0 };

    pub fn make_intervals(position: &Point4D, size: &Point4D) -> [Interval; 4] {
        [Interval { begin: position.x, end: position.x + size.x },
         Interval { begin: position.y, end: position.y + size.y },
         Interval { begin: position.z, end: position.z + size.z },
         Interval { begin: position.w, end: position.w + size.w }]
    }
}

impl fmt::Debug for Point4D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

impl Index<usize> for Point4D {
    type Output = IntType;

    fn index(&self, index: usize) -> &IntType {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Point4D index out of bounds.")
        }
    }
}

impl IndexMut<usize> for Point4D {
    fn index_mut(&mut self, index: usize) -> &mut IntType {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Point4D index out of bounds.")
        }
    }
}

pub fn product<T: Clone>(list: &[Vec<T>]) -> Vec<Vec<T>> {
    if list.len() == 0 {
        return vec!(vec!());
    }
    let mut prods: Vec<Vec<T>> = vec!();
    let rest = product(&list[1..]);
    for v in &list[0] {
        for prod in &rest {
            let mut new_prod = vec!(v.clone());
            new_prod.extend(prod.clone());
            prods.push(new_prod);
        }
    }
    prods
}

pub fn permutations<T: Clone>(list: &[T], k: usize) -> Vec<Vec<T>> {
    let n = list.len();
    if k == 0 || k > n {
        return vec!(vec!());
    }
    let mut perms: Vec<Vec<T>> = vec!();
    for (j, v) in list.iter().enumerate() {
        let mut rest = list.clone().to_vec();
        rest.remove(j);
        for perm in permutations(&rest, k - 1).iter() {
            let mut new_perm = vec!(v.clone());
            new_perm.extend(perm.clone());
            perms.push(new_perm);
        }
    }
    perms
}

pub fn combinations<T: Clone>(list: &[T], k: usize) -> Vec<Vec<T>> {
    let n = list.len();
    if k == 0 || k > n {
        return vec!(vec!());
    }
    if k == n {
        return vec!(list.to_vec());
    }
    let mut combs: Vec<Vec<T>> = vec!();
    combs.extend(combinations(&list[1..], k - 1).iter().map(|comb| {
        let mut new_combs = vec!(list[0].clone());
        new_combs.extend(comb.clone());
        new_combs
    }));
    combs.extend(combinations(&list[1..], k));
    combs
}

pub fn combinations_with_repetition<T: Clone>(list: &[T], k: usize) -> Vec<Vec<T>> {
    if k == 0 {
        return vec!(vec!());
    }
    let mut combs: Vec<Vec<T>> = vec!();
    let rest = combinations_with_repetition(list, k - 1);
    for v in list.iter() {
        for comb in rest.iter() {
            let mut new_comb = vec!(v.clone());
            new_comb.extend(comb.clone());
            combs.push(new_comb);
        }
    }
    combs
}

pub fn list_except<T: PartialEq + Clone>(list: &[T], exclude: &[T]) -> Vec<T> {
    list.iter().filter(|&v| !exclude.contains(v)).cloned().collect()
}
