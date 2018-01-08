use std::collections::HashMap;
use std::cmp::Ordering;

fn main() {
    println!("Representative dimension tuple set for n = 4:");
    representative_tuples_4d(100);
    println!("Representative dimension tuple set for n = 5:");
    representative_tuples_5d(200);
}

fn representative_tuples_4d(limit: usize) {
    let overlap_inequalities = [
        ([1, 4], [2, 3])
    ];
    let mut counts: HashMap<String, usize> = HashMap::new();
    for a in 1..limit {
        for b in a + 1..limit {
            for c in b + 1..limit {
                for d in c + 1..limit {
                    let dimension_tuple = [a, b, c, d];
                    if !satisfies_hoffmans_inequality(&dimension_tuple) {
                        continue;
                    }
                    let status: String = overlap_inequalities.iter().map(|&overlap_inequality| {
                        ordering_str(check_overlap_inequalities(&dimension_tuple, &overlap_inequality))
                    }).collect::<_>();
                    if !status.contains('=') {
                        let count = counts.entry(status.clone()).or_insert(0);
                        if *count == 0 {
                            println!("{}: {:?}", status, dimension_tuple);
                        }
                        *count += 1;
                    }
                }
            }
        }
    }
    println!("Total signatures: {}", counts.len());
    for (status, count) in &counts {
        println!("{}: {}", status, count);
    }
}

fn representative_tuples_5d(limit: usize) {
    let overlap_inequalities = [
        ([3, 4], [2, 5]),
        ([3, 4], [1, 5]),
        ([2, 4], [1, 5]),
        ([2, 3], [1, 5]),
        ([2, 3], [1, 4])
    ];
    let mut counts: HashMap<String, usize> = HashMap::new();
    for a in 1..limit {
        for b in a + 1..limit {
            for c in b + 1..limit {
                for d in c + 1..limit {
                    for e in d + 1..limit {
                        let dimension_tuple = [a, b, c, d, e];
                        if !satisfies_hoffmans_inequality(&dimension_tuple) {
                            continue;
                        }
                        let status: String = overlap_inequalities.iter().map(|&overlap_inequality| {
                            ordering_str(check_overlap_inequalities(&dimension_tuple, &overlap_inequality))
                        }).collect::<_>();
                        if !status.contains('=') {
                            let count = counts.entry(status.clone()).or_insert(0);
                            if *count == 0 {
                                println!("{}: {:?}", status, dimension_tuple);
                            }
                            *count += 1;
                        }
                    }
                }
            }
        }
    }
    println!("Total signatures: {}", counts.len());
    for (status, count) in &counts {
        println!("{}: {}", status, count);
    }
}

fn ordering_str(v: Ordering) -> char {
    match v {
        Ordering::Less => '<',
        Ordering::Equal => '=',
        Ordering::Greater => '>'
    }
}

fn satisfies_hoffmans_inequality(dimension_tuple: &[usize]) -> bool {
    dimension_tuple.iter().sum::<usize>() < (dimension_tuple.len() + 1) * dimension_tuple[0]
}

fn check_overlap_inequalities(dimension_tuple: &[usize], inequality: &([usize; 2], [usize; 2])) -> Ordering {
    let &(lhs, rhs) = inequality;
    let lhs_sum: usize = lhs.iter().map(|&i| dimension_tuple[i - 1]).sum();
    let rhs_sum: usize = rhs.iter().map(|&i| dimension_tuple[i - 1]).sum();
    lhs_sum.cmp(&rhs_sum)
}
