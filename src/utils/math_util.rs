#[derive(Clone, Copy, Debug)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn new(min: &f32, max: &f32) -> Self {
        Self {
            min: *min,
            max: *max,
        }
    }
    pub fn from_between(lhs: &Self, rhs: &Self) -> Self {
        Self {
            min: lhs.min.min(rhs.min),
            max: lhs.max.max(rhs.max),
        }
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
    const EMPTY: Self = Self { min: 0.0, max: 0.0 };
    const UNIVERSAL: Self = Self {
        min: -f32::INFINITY,
        max: f32::INFINITY,
    };
}

pub fn lerp(begin: f32, end: f32, time: f32) -> f32 {
    begin + time * (end - begin)
}

pub fn is_rectangular<T>(mat: &Vec<Vec<T>>) -> bool {
    let mut len = mat[0].len();
    for i in 0..mat.len() {
        if mat[i].len() != len {
            return false;
        }
        len = mat[i].len();
    }
    true
}
