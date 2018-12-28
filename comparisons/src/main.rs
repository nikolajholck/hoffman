extern crate hoffman;
extern crate colored;

use hoffman::*;
use colored::*;

use std::collections::HashMap;
use std::iter::repeat;
use std::cmp::Ordering;

#[derive(Clone, Copy, PartialEq)]
enum Order {
    Less,
    Equal,
    Greater,
    NA
}

fn main() {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";

    for n in 2..7/*alphabet.len()*/ {
        println!("n = {}", n);
        let brick = alphabet.chars().take(n).collect::<Vec<char>>();
        //println!("Brick: {:?}", brick);
        let mut tups: Vec<Vec<char>> = Vec::new();
        for i in 0..n + 1 {
            let combs = combinatorics::combinations(&brick, i);
            //println!("{}: {}", i, combs.len());
            tups.extend(combs);
        }
        let table_size = tups.len();
        //println!("Comparison table size: {:?}", tups.len());
        //println!("Tuples: {:?}", tups);

        let tup_sums: Vec<String> = tups.iter().map(|a| a.iter().collect()).collect();
        let mut indices: HashMap<String, usize> = HashMap::new();
        //println!("Tuples sums: {:?}", tup_sums);

        for (i, sum) in tup_sums.iter().enumerate() {
            indices.insert(sum.clone(), i);
        }
        //println!("Indices: {:?}", indices);

        let mut comparison_table: Vec<Vec<Option<Order>>> = repeat(repeat(Some(Order::NA)).take(table_size).collect::<Vec<_>>()).take(table_size).collect::<Vec<_>>();

        for (ai, a) in tups.iter().enumerate() {
            for (bi, b) in tups.iter().enumerate() {
                let (la, lb) = (a.len(), b.len());
                let val = match la.cmp(&lb) {
                    Ordering::Equal => {
                        let lt = a.iter().zip(b.iter()).all(|(x, y)| x <= y);
                        let gt = a.iter().zip(b.iter()).all(|(x, y)| x >= y);
                        match (lt, gt) {
                            (false, false) => None,
                            (true, false) => Some(Order::Less),
                            (true, true) => Some(Order::Equal),
                            (false, true) => Some(Order::Greater)
                        }
                    },
                    Ordering::Less => Some(Order::NA),
                    Ordering::Greater => Some(Order::NA)
                };
                if ai <= bi && val == None{
                    //println!("{:?} ? {:?}", a, b);
                }
                comparison_table[ai][bi] = val;
            }
        }
        println!("Unknowns: {}", count_unknown_comparisions(&comparison_table));
        print_comparison_table(&comparison_table);

    }

}

fn count_unknown_comparisions(table: &Vec<Vec<Option<Order>>>) -> usize {
    let mut unknowns = 0;
    for row in table.iter() {
        for cell in row.iter() {
            if *cell == None {
                unknowns += 1;
            }
        }
    }
    //assert!(unknowns % 2 == 0, "unknown comparisons doesn't come in pairs.");
    //unknowns / 2
    unknowns
}


fn print_comparison_table(table: &Vec<Vec<Option<Order>>>) {
    for row in table.iter() {
        for cell in row.iter() {
            print!("{} ", match *cell {
                None => "?".white().bold(),
                Some(Order::Less) => "<".red(),
                Some(Order::Equal) => "=".green(),
                Some(Order::Greater) => ">".blue(),
                Some(Order::NA) => ".".cyan()
            });
        }
        println!("");
    }
    println!("");
}
