use std::cmp::Ordering;
use std::fmt;
use std::ops::{Range as RangeOp, RangeFrom, RangeTo, RangeFull};

use super::bound::{BoundType, Bound};

// Unpack Bound and Range into scope to reduce verbosity
use super::bound::Bound::*;
use self::Range::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Interval<T> {
    lower: Bound<T>,
    upper: Bound<T>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Range<T> {
    Inhabited(Interval<T>),
    Empty,
}

impl<T> Range<T> {
    pub fn new(lower: Bound<T>, upper: Bound<T>) -> Range<T> {
        Inhabited(Interval {
            lower: lower,
            upper: upper,
        })
    }

    pub fn lower_inf(&self) -> bool {
        match *self {
            Inhabited(ref range) => match range.lower {
                Bound::Unbounded => true,
                _ => false,
            },
            Empty => false,
        }
    }

    pub fn upper_inf(&self) -> bool {
        match *self {
            Inhabited(ref range) => match range.upper {
                Bound::Unbounded => true,
                _ => false,
            },
            Empty => false,
        }
    }

    pub fn lower_inc(&self) -> bool {
        match *self {
            Inhabited(ref range) => match range.lower {
                Bound::Inclusive(_) => true,
                _ => false,
            },
            Empty => false,
        }
    }

    pub fn upper_inc(&self) -> bool {
        match *self {
            Inhabited(ref range) => match range.upper {
                Bound::Inclusive(_) => true,
                _ => false,
            },
            Empty => false,
        }
    }

    pub fn is_inhabited(&self) -> bool {
        match *self {
            Inhabited(_) => true,
            Empty => false,
        }
    }

    pub fn is_empty(&self) -> bool {
        !self.is_inhabited()
    }

    pub fn is_bounded(&self) -> bool {
        match *self {
            Inhabited(ref range) => {
                range.lower.is_bounded() && range.upper.is_bounded()
            },
            Empty => true,
        }
    }
}

impl<T: PartialOrd> Range<T> {
    pub fn contains(&self, item: &T) -> bool {
        match *self {
            Inhabited(ref range) => {
                let within_lower = match range.lower {
                    Inclusive(ref lower) => item >= lower,
                    Exclusive(ref lower) => item > lower,
                    Unbounded => true,
                };

                let within_upper = match range.upper {
                    Inclusive(ref upper) => item <= upper,
                    Exclusive(ref upper) => item < upper,
                    Unbounded => true,
                };

                within_lower && within_upper
            },
            Empty => false,
        }
    }
}

impl<T> From<RangeOp<Bound<T>>> for Range<T> {
    fn from(range: RangeOp<Bound<T>>) -> Range<T> {
        Range::new(range.start, range.end)
    }
}

impl<T> From<RangeFrom<Bound<T>>> for Range<T> {
    fn from(range: RangeFrom<Bound<T>>) -> Range<T> {
        Range::new(range.start, Unbounded)
    }
}

impl<T> From<RangeTo<Bound<T>>> for Range<T> {
    fn from(range: RangeTo<Bound<T>>) -> Range<T> {
        Range::new(Unbounded, range.end)
    }
}

impl<T> From<RangeFull> for Range<T> {
    #[allow(unused_variables)]
    fn from(range: RangeFull) -> Range<T> {
        Range::new(Unbounded, Unbounded)
    }
}

// Add shorthands for common range types such as Range::from(1..10). The lower
// end is always inclusive and the upper end is always exclusive.
macro_rules! impl_from_rangeops_for_range {
    ($($t:ty)*) => ($(
        impl From<RangeOp<$t>> for Range<$t> {
            fn from(range: RangeOp<$t>) -> Range<$t> {
                Range::from(Inclusive(range.start) .. Exclusive(range.end))
            }
        }

        impl From<RangeFrom<$t>> for Range<$t> {
            fn from(range: RangeFrom<$t>) -> Range<$t> {
                Range::new(Inclusive(range.start), Unbounded)
            }
        }

        impl From<RangeTo<$t>> for Range<$t> {
            fn from(range: RangeTo<$t>) -> Range<$t> {
                Range::new(Unbounded, Exclusive(range.end))
            }
        }
    )*)
}

impl_from_rangeops_for_range! { usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64 }

impl<T: Clone> Range<T> {
    pub fn lower(&self) -> Option<Bound<T>> {
        match *self {
            Inhabited(ref range) => Some(range.lower.clone()),
            Empty => None,
        }
    }

    pub fn upper(&self) -> Option<Bound<T>> {
        match *self {
            Inhabited(ref range) => Some(range.upper.clone()),
            Empty => None,
        }
    }
}

impl<T: fmt::Display> fmt::Display for Range<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Inhabited(ref range) => {
                match range.lower {
                    Bound::Inclusive(ref x) => write!(f, "[{}", x),
                    Bound::Exclusive(ref x) => write!(f, "({}", x),
                    Bound::Unbounded => write!(f, "("),
                }?;

                write!(f, ",")?;

                match range.upper {
                    Bound::Inclusive(ref x) => write!(f, "{}]", x),
                    Bound::Exclusive(ref x) => write!(f, "{})", x),
                    Bound::Unbounded => write!(f, ")"),
                }
            },
            Empty => write!(f, "(empty)"),
        }
    }
}

impl<T: PartialOrd> PartialOrd for Range<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (&Inhabited(ref a), &Inhabited(ref b)) => {
                match Bound::compare_bounds(
                    BoundType::Lower, &a.lower, &b.lower)
                {
                    Some(Ordering::Equal) => {
                        Bound::compare_bounds(
                            BoundType::Upper, &a.upper, &b.upper)
                    },
                    cmp => cmp,
                }
            },
            _ => None,
        }
    }
}
