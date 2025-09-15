use nalgebra::{Vector3};

type Vec3 = Vector3<f32>;

#[derive(Clone, Copy)]
pub struct Interval {
    min : f32, max: f32
}

impl Interval {
    pub fn new(min: &f32, max: &f32) -> Self {
        Self { min: *min, max: *max }
    }
    pub fn from_between(lhs: &Self, rhs: &Self) -> Self {
        Self {min: lhs.min.min(rhs.min), max: lhs.max.max(rhs.max)}
    }

    pub fn within(&self, x: &f32) -> bool {
        (self.min < *x) && (*x < self.max)
    }

    pub fn within_inc(&self, x: &f32) -> bool {
        (self.min <= *x) && (*x <= self.max)
    }

    pub fn clamp(&self, x: &f32) -> f32 {
        self.min.max(*x).min(self.max)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    origin : Vec3,
    direction: Vec3,
    time: f32
}

impl Ray {
    pub fn at(&self, time: &f32) -> Vec3 {
        self.origin + self.direction * (*time)
    }

    pub fn new(pos: &Vec3, dir: &Vec3, tm: &f32) -> Self {
        Self {origin: *pos, direction: *dir, time: *tm}
    }
}

