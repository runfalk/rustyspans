use std::cmp::Ordering;

// Unpack Bound and BoundType into scope to reduce verbosity
use Bound::*;
use BoundType::*;


#[derive(Clone, Debug, PartialEq)]
pub enum BoundType<'a, T: 'a> {
    Lower(&'a Bound<T>),
    Upper(&'a Bound<T>),
}

impl<'a, T: 'a + PartialOrd> PartialOrd for BoundType<'a, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (&Lower(ref a), &Lower(ref b)) => {
                match (*a, *b) {
                    (&Unbounded, &Unbounded) => Some(Ordering::Equal),
                    (&Unbounded, _) => Some(Ordering::Less),
                    (_, &Unbounded) => Some(Ordering::Greater),
                    (&Inclusive(ref a), &Inclusive(ref b)) |
                    (&Exclusive(ref a), &Exclusive(ref b)) => a.partial_cmp(&b),
                    (&Inclusive(ref a), &Exclusive(ref b)) => {
                        match a.partial_cmp(&b) {
                            Some(Ordering::Equal) => Some(Ordering::Less),
                            cmp => cmp,
                        }
                    },
                    (&Exclusive(ref a), &Inclusive(ref b)) => {
                        match a.partial_cmp(&b) {
                            Some(Ordering::Equal) => Some(Ordering::Greater),
                            cmp => cmp,
                        }
                    },
                }
            },
            (&Upper(ref a), &Upper(ref b)) => {
                match (*a, *b) {
                    (&Unbounded, &Unbounded) => Some(Ordering::Equal),
                    (&Unbounded, _) => Some(Ordering::Greater),
                    (_, &Unbounded) => Some(Ordering::Less),
                    (&Inclusive(ref a), &Inclusive(ref b)) |
                    (&Exclusive(ref a), &Exclusive(ref b)) => a.partial_cmp(&b),
                    (&Inclusive(ref a), &Exclusive(ref b)) => {
                        match a.partial_cmp(&b) {
                            Some(Ordering::Equal) => Some(Ordering::Greater),
                            cmp => cmp,
                        }
                    },
                    (&Exclusive(ref a), &Inclusive(ref b)) => {
                        match a.partial_cmp(&b) {
                            Some(Ordering::Equal) => Some(Ordering::Less),
                            cmp => cmp,
                        }
                    },
                }
            },
            (&Lower(ref a), &Upper(ref b)) => {
                match (*a, *b) {
                    (&Unbounded, _) | (_, &Unbounded) => Some(Ordering::Less),
                    (&Inclusive(ref a), &Inclusive(ref b)) => {
                        match a.partial_cmp(&b) {
                            Some(Ordering::Equal) => Some(Ordering::Less),
                            cmp => cmp,
                        }
                    }
                    (&Exclusive(ref a), &Exclusive(ref b)) => {
                        match a.partial_cmp(&b) {
                            Some(Ordering::Equal) => Some(Ordering::Greater),
                            cmp => cmp,
                        }
                    },
                    (&Inclusive(ref a), &Exclusive(ref b)) |
                    (&Exclusive(ref a), &Inclusive(ref b)) => a.partial_cmp(&b),
                }
            },
            (&Upper(ref a), &Lower(ref b)) => {
                match (*a, *b) {
                    (&Unbounded, _) | (_, &Unbounded) => Some(Ordering::Greater),
                    (&Inclusive(ref a), &Inclusive(ref b)) => {
                        match a.partial_cmp(&b) {
                            Some(Ordering::Equal) => Some(Ordering::Greater),
                            cmp => cmp,
                        }
                    }
                    (&Exclusive(ref a), &Exclusive(ref b)) => {
                        match a.partial_cmp(&b) {
                            Some(Ordering::Equal) => Some(Ordering::Less),
                            cmp => cmp,
                        }
                    },
                    (&Inclusive(ref a), &Exclusive(ref b)) |
                    (&Exclusive(ref a), &Inclusive(ref b)) => a.partial_cmp(&b),
                }
            },
        }
    }
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
            Unbounded => false,
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    include!("test_bound.rs");
}
