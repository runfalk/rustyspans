#![deny(dead_code)]
extern crate rustyspans;

use std::cmp::Ordering;

use rustyspans::Range;
use rustyspans::Bound::*;

#[test]
fn from_i32_range() {
    assert_eq!(
        Range::new(Inclusive(1), Exclusive(10)),
        Range::from(1..10));
}

#[test]
fn lower_inf() {
    assert!(!Range::Empty::<i32>.lower_inf());
    assert!(!Range::from(1..10).lower_inf());
    assert!(!Range::from(1..).lower_inf());
    assert!(Range::from(..10).lower_inf());
    assert!(Range::<i32>::from(..).lower_inf());
}

#[test]
fn upper_inf() {
    assert!(!Range::Empty::<i32>.upper_inf());
    assert!(!Range::from(1..10).upper_inf());
    assert!(Range::from(1..).upper_inf());
    assert!(!Range::from(..10).upper_inf());
    assert!(Range::<i32>::from(..).upper_inf());
}

#[test]
fn lower_inc() {
    assert!(!Range::Empty::<i32>.lower_inc());
    assert!(!Range::<i32>::from(..).lower_inc());
    assert!(Range::from(1..).lower_inc());
    assert!(!Range::from(Exclusive(1)..).lower_inc());
}

#[test]
fn upper_inc() {
    assert!(!Range::Empty::<i32>.upper_inc());
    assert!(!Range::<i32>::from(..).upper_inc());
    assert!(!Range::from(..10).upper_inc());
    assert!(Range::from(..Inclusive(10)).upper_inc());
}

#[test]
fn is_bounded() {
    assert!(Range::Empty::<i32>.is_bounded());
    assert!(Range::from(1..10).is_bounded());
    assert!(!Range::from(1..).is_bounded());
    assert!(!Range::from(..10).is_bounded());
    assert!(!Range::<i32>::from(..).is_bounded());
}

#[test]
fn contains() {
    assert!(!Range::from(1..10).contains(&0));
    assert!(Range::from(1..10).contains(&1));
    assert!(Range::from(1..10).contains(&9));
    assert!(!Range::from(1..10).contains(&10));
}

#[test]
fn fmt_display() {
    assert_eq!("(empty)", format!("{}", Range::Empty::<i32>));
    assert_eq!("(,)", format!("{}", Range::<i32>::from(..)));

    assert_eq!("[1,10)", format!("{}", Range::from(1..10)));
    assert_eq!("[1,)", format!("{}", Range::from(1..)));
    assert_eq!("(,10)", format!("{}", Range::from(..10)));

    assert_eq!("(1,10]", format!("{}", Range::from(Exclusive(1)..Inclusive(10))));
}

#[test]
fn partial_cmp() {
    let undef: Option<Ordering> = None;
    let lt = Some(Ordering::Less);
    let eq = Some(Ordering::Equal);
    let gt = Some(Ordering::Greater);

    let empty = Range::Empty::<i32>;
    let unbounded = Range::<i32>::from(..);
    let r1_10 = Range::from(1..10);
    let r1_100 = Range::from(1..100);
    let r1_ = Range::from(1..);
    let r_10 = Range::from(..10);

    assert_eq!(undef, empty.partial_cmp(&empty));
    assert_eq!(undef, empty.partial_cmp(&unbounded));
    assert_eq!(undef, empty.partial_cmp(&r1_10));
    assert_eq!(undef, empty.partial_cmp(&r1_100));
    assert_eq!(undef, empty.partial_cmp(&r1_));
    assert_eq!(undef, empty.partial_cmp(&r_10));

    assert_eq!(eq, unbounded.partial_cmp(&unbounded));
    assert_eq!(lt, unbounded.partial_cmp(&r1_10));
    assert_eq!(lt, unbounded.partial_cmp(&r1_100));
    assert_eq!(lt, unbounded.partial_cmp(&r1_));
    assert_eq!(gt, unbounded.partial_cmp(&r_10));

    assert_eq!(eq, r1_10.partial_cmp(&r1_10));
    assert_eq!(lt, r1_10.partial_cmp(&r1_100));
    assert_eq!(gt, r1_100.partial_cmp(&r1_10));
    assert_eq!(gt, r1_10.partial_cmp(&r_10));
    assert_eq!(lt, r1_10.partial_cmp(&r1_));
}
