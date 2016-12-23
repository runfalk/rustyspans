use std::cmp::Ordering;
use std::fmt;
use std::ops::Range as RangeOp;

use super::bound::{BoundType, Bound};

// Unpack Bound into scope to reduce verbosity
use super::bound::Bound::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Range<T> {
    lower: Bound<T>,
    upper: Bound<T>,
}

impl<T> Range<T> {
    pub fn new(lower: Bound<T>, upper: Bound<T>) -> Range<T> {
        Range {
            lower: lower,
            upper: upper,
        }
    }

    pub fn lower_inf(&self) -> bool {
        match self.lower {
            Bound::Unbounded => true,
            _ => false,
        }
    }

    pub fn upper_inf(&self) -> bool {
        match self.upper {
            Bound::Unbounded => true,
            _ => false,
        }
    }

    pub fn is_bounded(&self) -> bool {
        self.lower.is_bounded() && self.upper.is_bounded()
    }
}

impl<T> From<RangeOp<Bound<T>>> for Range<T> {
    fn from(range: RangeOp<Bound<T>>) -> Range<T> {
        Range {
            lower: range.start,
            upper: range.end,
        }
    }
}

// Add shorthands for common range types such as Range::from(1..10). The lower
// end is always inclusive and the upper end is always exclusive.
macro_rules! from_rangeop_impl_for_range {
    ($($t:ty)*) => ($(
        impl From<RangeOp<$t>> for Range<$t> {
            fn from(range: RangeOp<$t>) -> Range<$t> {
                Range::from(Inclusive(range.start) .. Exclusive(range.end))
            }
        }
    )*)
}

from_rangeop_impl_for_range! { usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64 }

impl<T: Clone> Range<T> {
    pub fn lower(&self) -> Bound<T> {
        self.lower.clone()
    }

    pub fn upper(&self) -> Bound<T> {
        self.upper.clone()
    }
}

impl<T: fmt::Display> fmt::Display for Range<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.lower {
            Bound::Inclusive(ref x) => write!(f, "[{}", x),
            Bound::Exclusive(ref x) => write!(f, "({}", x),
            Bound::Unbounded => write!(f, "("),
        }?;

        write!(f, ",")?;

        match self.upper {
            Bound::Inclusive(ref x) => write!(f, "{}]", x),
            Bound::Exclusive(ref x) => write!(f, "{})", x),
            Bound::Unbounded => write!(f, ")"),
        }
    }
}

impl<T: PartialOrd> PartialOrd for Range<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match Bound::compare_bounds(BoundType::Lower, &self.lower, &other.lower) {
            Some(Ordering::Equal) => {
                Bound::compare_bounds(BoundType::Upper, &self.upper, &other.upper)
            },
            cmp => cmp,
        }
    }
}
