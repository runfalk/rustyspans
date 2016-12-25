use std::cmp::Ordering;

// Unpack Bound and BoundType into scope to reduce verbosity
use Bound::*;
use BoundType::*;

#[derive(Clone, Debug, PartialEq)]
pub enum BoundType {
    Lower,
    Upper,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Bound<T> {
    Inclusive(T),
    Exclusive(T),
    Unbounded,
}

impl<T> Bound<T> {
    pub fn is_bounded(&self) -> bool {
        match *self {
            Bound::Unbounded => false,
            _ => true,
        }
    }
}

impl<T: PartialOrd> Bound<T> {
    pub fn compare_bounds(a_type: BoundType, b_type: BoundType, a: &Bound<T>, b: &Bound<T>) -> Option<Ordering> {
        match (a, b) {
            (&Unbounded, &Unbounded) => Some(match (a_type, b_type) {
                (Lower, Lower) | (Upper, Upper) => Ordering::Equal,
                (Lower, Upper) => Ordering::Less,
                (Upper, Lower) => Ordering::Greater,
            }),
            (&Unbounded, &Inclusive(_)) |
            (&Unbounded, &Exclusive(_)) => Some(match (a_type, b_type) {
                (Lower, _) => Ordering::Less,
                (Upper, _) => Ordering::Greater,
            }),
            (&Inclusive(_), &Unbounded) |
            (&Exclusive(_), &Unbounded) => Some(match (a_type, b_type) {
                (_, Lower) => Ordering::Greater,
                (_, Upper) => Ordering::Less,
            }),
            (&Inclusive(ref a), &Inclusive(ref b)) => {
                match a.partial_cmp(&b) {
                    Some(Ordering::Equal) => Some(match (a_type, b_type) {
                        (Lower, Lower) | (Upper, Upper) => Ordering::Equal,
                        (Lower, Upper) => Ordering::Greater,
                        (Upper, Lower) => Ordering::Less,
                    }),
                    cmp => cmp,
                }
            },
            (&Exclusive(ref a), &Exclusive(ref b)) => {
                match a.partial_cmp(&b) {
                    Some(Ordering::Equal) => Some(match (a_type, b_type) {
                        (Lower, Lower) | (Upper, Upper) => Ordering::Equal,
                        (Lower, Upper) => Ordering::Less,
                        (Upper, Lower) => Ordering::Greater,
                    }),
                    cmp => cmp,
                }
            },
            (&Inclusive(ref a), &Exclusive(ref b)) => {
                match a.partial_cmp(&b) {
                    Some(Ordering::Equal) => Some(match (a_type, b_type) {
                        (_, Lower) => Ordering::Less,
                        (_, Upper) => Ordering::Greater,
                    }),
                    cmp => cmp,
                }
            },
            (&Exclusive(ref a), &Inclusive(ref b)) => {
                match a.partial_cmp(&b) {
                    Some(Ordering::Equal) => Some(match (a_type, b_type) {
                        (_, Lower) => Ordering::Greater,
                        (_, Upper) => Ordering::Less,
                    }),
                    cmp => cmp,
                }
            }
        }
    }
}
