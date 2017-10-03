#[cfg(test)]
mod tests;

pub mod cube;

use std::collections::HashMap;
use std::cmp::Ordering;

use super::*;

pub const N: usize = 3;
pub const COMPARISON_TABLE_SIZE: usize = 8;

pub type Brick = [IntType; N];

pub struct Comparator {
    indices: HashMap<IntType, usize>,
    table: [[Option<Ordering>; COMPARISON_TABLE_SIZE]; COMPARISON_TABLE_SIZE],
}

impl Comparator {

    pub fn constructor(brick: &Brick) -> Comparator {

        let mut tups: Vec<Vec<IntType>> = Vec::new();
        for i in 0..N + 1 {
            tups.extend(combinations(brick, i));
        }
        assert!(COMPARISON_TABLE_SIZE == tups.len(), "Unexpected comparison table size.");

        let tup_sums: Vec<IntType> = tups.iter().map(|a| a.iter().sum()).collect();
        let mut indices: HashMap<IntType, usize> = HashMap::new();

        for (i, sum) in tup_sums.iter().enumerate() {
            indices.insert(*sum, i);
        }

        let mut comparison_table = [[None; COMPARISON_TABLE_SIZE]; COMPARISON_TABLE_SIZE];

        for (ai, a) in tups.iter().enumerate() {
            for (bi, b) in tups.iter().enumerate() {
                let (la, lb) = (a.len(), b.len());
                comparison_table[ai][bi] = match la.cmp(&lb) {
                    Ordering::Equal => {
                        let a_sym = Comparator::symbolize(brick, &a);
                        let b_sym = Comparator::symbolize(brick, &b);
                        let lt = a_sym.iter().zip(b_sym.iter()).all(|(x, y)| x <= y);
                        let gt = a_sym.iter().zip(b_sym.iter()).all(|(x, y)| x >= y);
                        match (lt, gt) {
                            (false, false) => None,
                            (true, false) => Some(Ordering::Less),
                            (true, true) => Some(Ordering::Equal),
                            (false, true) => Some(Ordering::Greater)
                        }
                    },
                    Ordering::Less => Some(Ordering::Less),
                    Ordering::Greater => Some(Ordering::Greater)
                };
            }
        }

        Comparator {
            indices: indices,
            table: comparison_table
        }
    }

    fn compare(&self, x: IntType, y: IntType) -> Option<Ordering> {
        let (x_index, y_index) = (self.indices[&x], self.indices[&y]);
        self.table[x_index][y_index]
    }

    /*  Returns true unless sure that the intervals (a) and (b) don't intersect. */
    fn intervals_intersect(&self, a: &Interval, b: &Interval) -> bool {
        if a.is_zero() || b.is_zero() { return false }
        // Logic: if a.begin < b.begin { b.begin < a.end } else { a.begin < b.end }
        if let Some(result) = self.compare(a.begin, b.begin) {
            let order = if result == Ordering::Less { self.compare(b.begin, a.end) } else { self.compare(a.begin, b.end) };
            if let Some(order) = order {
                return order == Ordering::Less;
            }
        }
        true // Case: Unknown, so we'll assume overlap.
    }

    const ALPHABET: [char; N] = ['a', 'b', 'c'];

    fn symbolize(brick: &Brick, list: &[IntType]) -> Vec<char> {
        let mut map: HashMap<IntType, char> = HashMap::new();
        for (v, c) in brick.iter().zip(Comparator::ALPHABET.iter()) {
            map.insert(*v, *c);
        }
        list.iter().map(|v| map[&v]).collect()
    }

}
