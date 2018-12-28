
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
        for perm in &permutations(&rest, k - 1) {
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
    for v in list {
        for comb in &rest {
            let mut new_comb = vec!(v.clone());
            new_comb.extend(comb.clone());
            combs.push(new_comb);
        }
    }
    combs
}

pub fn factorial(n: usize) -> usize {
    if n <= 1 { 1 } else { n * factorial(n - 1) }
}
