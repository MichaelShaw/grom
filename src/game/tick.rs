use std::ops::{Add, Sub};
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Tick {
    pub at: u64
}

impl Add<Tick> for Tick {
    type Output = Tick;
    fn add(self, other: Tick) -> Tick {
        tick(self.at + other.at)
    }
}

impl Sub<Tick> for Tick {
    type Output = Tick;
    fn sub(self, other: Tick) -> Tick {
        tick(self.at - other.at)
    }
}

impl PartialOrd for Tick {
    fn partial_cmp(&self, other: &Tick) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Tick {
    fn cmp(&self, other: &Tick) -> Ordering {
        self.at.cmp(&other.at)
    }
}

impl Tick {
    pub fn plus(&self, n:u64) -> Tick {
        tick(self.at + n)
    }

    pub fn succ(&self) -> Tick {
        tick(self.at + 1)
    }

    pub fn pred(&self) -> Tick {
        tick(self.at - 1)
    }
}


pub fn tick(at:u64) -> Tick {
    Tick { at: at }
}