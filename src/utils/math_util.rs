use nalgebra::Vector3;

type Vec3 = Vector3<f32>;
type Point3 = Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Interval {
    min: f32,
    max: f32,
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

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
    time: f32,
}

impl Ray {
    pub fn at(&self, time: &f32) -> Vec3 {
        self.origin + self.direction * (*time)
    }

    pub fn new(pos: &Vec3, dir: &Vec3, tm: &f32) -> Self {
        Self {
            origin: *pos,
            direction: *dir,
            time: *tm,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct AABB {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl AABB {
    pub fn from_pt(a: &Point3, b: &Point3) -> Self {
        Self {
            x: Interval { min: a.x, max: b.x },
            y: Interval { min: a.y, max: b.y },
            z: Interval { min: a.z, max: b.z },
        }
    }

    pub fn from_interval(x: &Interval, y: &Interval, z: &Interval) -> Self {
        Self {
            x: *x,
            y: *y,
            z: *z,
        }
    }

    pub fn from_merge(up: &Self, down: &Self) -> Self {
        Self {
            x: Interval::from_between(&up.x, &down.x),
            y: Interval::from_between(&up.y, &down.y),
            z: Interval::from_between(&up.z, &down.z),
        }
    }
    pub fn axis(&self, axis: i32) -> Interval {
        match (axis) {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("invalid axis argument, can only be between 0 and 2"),
        }
    }
    pub fn hit(&self, ray: &Ray, ray_int: &Interval) -> bool {
        for i in 0..3 {
            let inverse_dir = ray.direction[i];
            let t = 
        }
    }
}
