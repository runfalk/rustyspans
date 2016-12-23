use std::cmp::Ordering;

// Unpack Bound into scope to reduce verbosity
use Bound::*;

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
    pub fn compare_bounds(type_: BoundType, a: &Bound<T>, b: &Bound<T>) -> Option<Ordering> {
        #[inline]
        fn invert_order(cmp: Ordering) -> Ordering {
            match cmp {
                Ordering::Less => Ordering::Greater,
                Ordering::Equal => Ordering::Equal,
                Ordering::Greater => Ordering::Less,
            }
        }

        // The relation between an unbounded value and a bounded one
        let unbounded_order = match type_ {
            BoundType::Lower => Ordering::Less,
            BoundType::Upper => Ordering::Greater,
        };

        match (a, b) {
            (&Unbounded, &Unbounded) => Some(Ordering::Equal),
            (&Unbounded, &Inclusive(_)) |
            (&Unbounded, &Exclusive(_)) => Some(unbounded_order),
            (&Inclusive(_), &Unbounded) |
            (&Exclusive(_), &Unbounded) => Some(invert_order(unbounded_order)),
            (&Inclusive(ref a), &Inclusive(ref b)) |
            (&Exclusive(ref a), &Exclusive(ref b)) => a.partial_cmp(&b),
            (&Inclusive(ref a), &Exclusive(ref b)) => {
                match a.partial_cmp(&b) {
                    Some(Ordering::Equal) => Some(unbounded_order),
                    cmp => cmp,
                }
            },
            (&Exclusive(ref a), &Inclusive(ref b)) => {
                match a.partial_cmp(&b) {
                    Some(Ordering::Equal) => Some(invert_order(unbounded_order)),
                    cmp => cmp,
                }
            }
        }
    }
}
