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

impl From<RangeOp<i32>> for Range<i32> {
    fn from(range: RangeOp<i32>) -> Range<i32> {
        Range::from(Inclusive(range.start) .. Exclusive(range.end))
    }
}

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
