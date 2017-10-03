use super::*;

#[test]
fn interval_is_zero() {
    let a = Interval { begin: 2, end: 7 };
    let b = Interval { begin: 7, end: 2 };
    let c = Interval { begin: 5, end: 5 };
    let z = Interval { begin: 0, end: 0 };
    let z1 = Interval { begin: 0, end: 2 };
    let z2 = Interval { begin: 2, end: 0 };

    assert!(!a.is_zero());
    assert!(!b.is_zero());
    assert!(!c.is_zero());
    assert!(z.is_zero());
    assert!(!z1.is_zero());
    assert!(!z2.is_zero());
}

#[test]
fn interval_is_degenerate() {
    let a = Interval { begin: 2, end: 7 };
    let b = Interval { begin: 7, end: 2 };
    let c = Interval { begin: 5, end: 5 };
    let z = Interval { begin: 0, end: 0 };
    let z1 = Interval { begin: 0, end: 2 };
    let z2 = Interval { begin: 2, end: 0 };

    assert!(!a.is_degenerate());
    assert!(b.is_degenerate());
    assert!(c.is_degenerate());
    assert!(z.is_degenerate());
    assert!(!z1.is_degenerate());
    assert!(z2.is_degenerate());
}

#[test]
fn point2d_make_intervals() {
    let a = Point2D { x: 2, y: 3 };
    let b = Point2D { x: 5, y: 8 };
    let intervals = [Interval { begin: 2, end: 2 + 5 }, Interval { begin: 3, end: 3 + 8 }];
    assert_eq!(Point2D::make_intervals(&a, &b), intervals);
    assert_ne!(Point2D::make_intervals(&b, &a), intervals);
}

#[test]
fn permutations_test() {
    let a = [1, 2, 3, 4];
    let b = [76, 76, 0];
    assert!(permutations(&a, 4).len() == 4 * 3 * 2 * 1);
    assert!(permutations(&a, 3).len() == 4 * 3 * 2);
    assert!(permutations(&a, 2).len() == 4 * 3);
    assert!(permutations(&a, 1).len() == 4);
    assert_eq!(permutations(&a, 2), [[1, 2], [1, 3], [1, 4], [2, 1], [2, 3], [2, 4], [3, 1], [3, 2], [3, 4], [4, 1], [4, 2], [4, 3]]);

    assert!(permutations(&b, 1).len() == 3);
    assert_eq!(permutations(&b, 1), [[76], [76], [0]]);
    assert_eq!(permutations(&b, 0), [[]]);
}

#[test]
fn combinations_test() {
    let a = [1, 2, 3, 4];
    let b = [76, 76, 0];
    assert!(combinations(&a, 4).len() == 1);
    assert!(combinations(&a, 3).len() == 4);
    assert!(combinations(&a, 2).len() == 6);
    assert!(combinations(&a, 1).len() == 4);
    assert_eq!(combinations(&a, 2), [[1, 2], [1, 3], [1, 4], [2, 3], [2, 4], [3, 4]]);

    assert!(combinations(&b, 0).len() == 1);
    assert!(combinations(&b, 1).len() == 3);
    assert!(combinations(&b, 2).len() == 3);
    assert!(combinations(&b, 3).len() == 1);
    assert_eq!(combinations(&b, 0), [[]]);
    assert_eq!(combinations(&b, 1), [[76], [76], [0]]);
    assert_eq!(combinations(&b, 2), [[76, 76], [76, 0], [76, 0]]);
    assert_eq!(combinations(&b, 3), [[76, 76, 0]]);
}

#[test]
fn combinations_with_repetition_test() {
    let a = [1, 2, 3, 4];
    let b = [76, 76, 0];
    let c = [0, 1];
    assert!(combinations_with_repetition(&a, 4).len() == 256);
    assert!(combinations_with_repetition(&a, 3).len() == 64);
    assert!(combinations_with_repetition(&a, 2).len() == 16);
    assert!(combinations_with_repetition(&a, 1).len() == 4);
    assert_eq!(combinations_with_repetition(&a, 2), [[1, 1], [1, 2], [1, 3], [1, 4],
                                                     [2, 1], [2, 2], [2, 3], [2, 4],
                                                     [3, 1], [3, 2], [3, 3], [3, 4],
                                                     [4, 1], [4, 2], [4, 3], [4, 4]]);

    assert!(combinations_with_repetition(&b, 0).len() == 1);
    assert!(combinations_with_repetition(&b, 1).len() == 3);
    assert!(combinations_with_repetition(&b, 2).len() == 9);
    assert!(combinations_with_repetition(&b, 3).len() == 27);
    assert_eq!(combinations_with_repetition(&c, 0), [[]]);
    assert_eq!(combinations_with_repetition(&c, 1), [[0], [1]]);
    assert_eq!(combinations_with_repetition(&c, 2), [[0, 0], [0, 1], [1, 0], [1, 1]]);
    assert_eq!(combinations_with_repetition(&c, 3), [[0, 0, 0], [0, 0, 1], [0, 1, 0], [0, 1, 1],
                                                     [1, 0, 0], [1, 0, 1], [1, 1, 0], [1, 1, 1]]);
}

#[test]
fn list_except_test() {
    let a = [1, 2, 3, 4];
    let b = [76, 76, 0];
    let c = [2, 4, 6];
    assert_eq!(list_except(&a, &b), a);
    assert_eq!(list_except(&a, &c), [1, 3]);
    assert_eq!(list_except(&c, &a), [6]);
}
