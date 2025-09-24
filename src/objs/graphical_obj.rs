use crate::objs::typedef::{Point3, Vec3};
use std::mem::swap;

use crate::utils::math_util::Interval;

#[derive(Clone, Copy)]
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
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
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
        match axis {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("invalid axis argument, can only be between 0 and 2"),
        }
    }
    pub fn hit(&self, ray: &Ray, ray_int: &Interval) -> bool {
        let mut ray_int_mut = *ray_int;
        for i in 0..3 {
            let inverse_dir = ray.direction[i];
            let origin = ray.origin[i];
            let mut t0 = (self.axis(i as i32).min - origin) * inverse_dir;
            let mut t1 = (self.axis(i as i32).max - origin) * inverse_dir;
            if inverse_dir < 0.0 {
                swap(&mut t0, &mut t1);
            }
            if t0 > ray_int_mut.min {
                ray_int_mut.min = t0;
            }
            if t1 < ray_int_mut.max {
                ray_int_mut.max = t1;
            }
            if ray_int_mut.max < ray_int_mut.min {
                return false;
            }
        }
        return true;
    }
}
