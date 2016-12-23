extern crate rustyspans;

use rustyspans::Range;
use rustyspans::Bound::*;

#[test]
fn from_i32_range() {
    assert_eq!(
        Range::new(Inclusive(1), Exclusive(10)),
        Range::from(1..10));
}

#[test]
fn contains() {
    assert!(!Range::from(1..10).contains(&0));
    assert!(Range::from(1..10).contains(&1));
    assert!(Range::from(1..10).contains(&9));
    assert!(!Range::from(1..10).contains(&10));
}
