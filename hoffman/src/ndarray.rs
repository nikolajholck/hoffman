use super::*;
use std::iter::Iterator;

#[derive(Clone, PartialEq)]
pub struct NdArray<T> {
    n: usize,
    shape: Shape,
    coords: Vec<Coord>,
    array: Vec<Option<T>>
}

impl<T: Clone + PartialEq> NdArray<T> {

    pub fn new(shape: &Shape) -> Self {
        Self {
            n: shape.len(),
            shape: shape.clone(),
            coords: utils::make_coords(shape),
            array: vec![None; shape.iter().product()]
        }
    }

    pub fn index(&self, coord: &Coord) -> usize {
        assert!(coord.len() == self.n);
        self.shape.iter().zip(coord.iter()).fold(0, |acc, (m, i)| acc * m + i )
    }

    pub fn coord(&self, index: usize) -> &Coord {
        &self.coords[index]
    }

    pub fn contains_key(&self, coord: &Coord) -> bool {
        self.array[self.index(coord)].is_some()
    }

    pub fn get(&self, coord: &Coord) -> Option<&T> {
        self.array[self.index(coord)].as_ref()
    }

    pub fn insert(&mut self, coord: &Coord, value: T) {
        let index = self.index(&coord);
        self.array[index] = Some(value);
    }

    pub fn remove(&mut self, coord: &Coord) -> Option<T> {
        let index = self.index(&coord);
        self.array[index].take()
    }

    pub fn differences(&self, other: &Self) -> usize {
        assert!(self.shape == other.shape);
        assert!(self.array.len() == other.array.len());
        self.array.iter().zip(other.array.iter()).filter(|&(x, y)| {
            x != y
        }).count()
    }

    pub fn n(&self) -> usize {
        self.n
    }

    pub fn shape(&self) -> &Shape {
        &self.shape
    }

    pub fn coords(&self) -> &Vec<Coord> {
        &self.coords
    }

    pub fn map<F>(&self, f: F) -> Self
    where F: FnMut((&Coord, &T)) -> (Coord, T) {
        let mut res = Self::new(&self.shape);
        for (coord, v) in self.iter().map(f) {
            res.insert(&coord, v)
        }
        res
    }

    pub fn iter(&self) -> NdArrayIter<T> {
        NdArrayIter {
            array: &self,
            current_index: 0
        }
    }
}

pub struct NdArrayIter<'a, T> {
    array: &'a NdArray<T>,
    current_index: usize
}

impl<'a, T: Clone + PartialEq> Iterator for NdArrayIter<'a, T> {
    type Item = (&'a Coord, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let length = self.array.array.len();
        while self.current_index < length {
            let coord = self.array.coord(self.current_index);
            let entry = &self.array.array[self.current_index];
            self.current_index += 1;
            if let Some(v) = entry {
                return Some((coord, v));
            }
        }
        None
    }
}
