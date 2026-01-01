use core::f64;

pub struct Interval {
    pub max: f64,
    pub min: f64,
}

impl Interval {
    pub fn new_from_values(max: f64, min: f64) -> Interval {
        Interval { max, min }
    }

    pub fn size(&self) -> f64 {
        return self.max - self.min;
    }

    pub fn contains(&self, x: f64) -> bool {
        return self.min <= x && x <= self.max;
    }

    pub fn surrounds(&self, x: f64) -> bool {
        return self.min < x && x < self.max;
    }
}

pub const UNIVERSE: Interval = Interval {
    min: -f64::INFINITY,
    max: f64::INFINITY,
};

pub const EMPTY: Interval = Interval {
    min: f64::INFINITY,
    max: -f64::INFINITY,
};
