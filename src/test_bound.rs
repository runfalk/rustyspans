use std::cmp::Ordering;

use super::{BoundType, Bound};
use super::Bound::*;

#[test]
fn test_equality() {
    assert_eq!(Unbounded::<i32>, Unbounded);
    assert_eq!(Inclusive(1), Inclusive(1));
    assert_eq!(Exclusive(1), Exclusive(1));

    assert!(Inclusive(1) != Unbounded);
    assert!(Inclusive(1) != Exclusive(1));

    assert!(Exclusive(1) != Unbounded);
    assert!(Exclusive(1) != Inclusive(1));
}

#[test]
fn test_compare_bounds_lower() {
    fn cmp<T: PartialOrd>(a: &Bound<T>, b: &Bound<T>) -> Ordering {
        BoundType::Lower(&a).partial_cmp(&BoundType::Lower(&b)).unwrap()
    }

    assert_eq!(Ordering::Equal, cmp::<i32>(&Unbounded, &Unbounded));
    assert_eq!(Ordering::Equal, cmp(&Inclusive(1), &Inclusive(1)));
    assert_eq!(Ordering::Equal, cmp(&Exclusive(1), &Exclusive(1)));

    assert_eq!(Ordering::Less, cmp(&Unbounded, &Inclusive(1)));
    assert_eq!(Ordering::Less, cmp(&Unbounded, &Exclusive(1)));

    assert_eq!(Ordering::Greater, cmp(&Inclusive(1), &Unbounded));
    assert_eq!(Ordering::Greater, cmp(&Exclusive(1), &Unbounded));

    assert_eq!(Ordering::Less, cmp(&Inclusive(1), &Exclusive(1)));
    assert_eq!(Ordering::Greater, cmp(&Exclusive(1), &Inclusive(1)));
}

#[test]
fn test_compare_bounds_upper() {
    fn cmp<T: PartialOrd>(a: &Bound<T>, b: &Bound<T>) -> Ordering {
        BoundType::Upper(&a).partial_cmp(&BoundType::Upper(&b)).unwrap()
    }

    assert_eq!(Ordering::Equal, cmp::<i32>(&Unbounded, &Unbounded));
    assert_eq!(Ordering::Equal, cmp(&Inclusive(1), &Inclusive(1)));
    assert_eq!(Ordering::Equal, cmp(&Exclusive(1), &Exclusive(1)));

    assert_eq!(Ordering::Greater, cmp(&Unbounded, &Inclusive(1)));
    assert_eq!(Ordering::Greater, cmp(&Unbounded, &Exclusive(1)));

    assert_eq!(Ordering::Less, cmp(&Inclusive(1), &Unbounded));
    assert_eq!(Ordering::Less, cmp(&Exclusive(1), &Unbounded));

    assert_eq!(Ordering::Greater, cmp(&Inclusive(1), &Exclusive(1)));
    assert_eq!(Ordering::Less, cmp(&Exclusive(1), &Inclusive(1)));
}
