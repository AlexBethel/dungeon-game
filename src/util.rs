//! Miscellanous utility functions and types used in other files.

use std::ops::Add;

use float_ord::FloatOrd;
use pathfinding::num_traits::Zero;

/// A somewhat more well-behaved floating point type, used in
/// pathfinding. Fully ordered, implements Eq, and has a defined zero
/// element.
#[derive(Clone, Copy)]
pub struct NiceFloat(pub f64);

impl PartialEq for NiceFloat {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for NiceFloat {}

impl PartialOrd for NiceFloat {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NiceFloat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // TODO: This is the /only/ use of FloatOrd in the program at
        // this point. It's kinda silly to bring in a dependency for
        // this; we should probably just implement the logic here
        // instead.
        FloatOrd(self.0).cmp(&FloatOrd(other.0))
    }
}

impl Add for NiceFloat {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Zero for NiceFloat {
    fn zero() -> Self {
        Self(0.0)
    }

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}
