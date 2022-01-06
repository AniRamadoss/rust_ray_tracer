use std::ops;
use crate::rtweekend;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    e: [f32;3],
}
impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        let vec = Vec3{
            e: [e0, e1, e2],
        };
        return vec;
    }

    pub fn x(&self) -> f32{
        return self.e[0];
    }

    pub fn y(&self) -> f32{
        return self.e[1];
    }

    pub fn z(&self) -> f32 {
        return self.e[2];
    }

    pub fn length(&self) -> f32 {
        let length =  ((self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]) as f32).sqrt() as f32;
        return length;
    }

    pub fn length_squared(&self) -> f32 {
        return self.length() * self.length();
    }

    #[inline]
    pub fn dot(self, other: Vec3) -> f32 {
        return self.e[0] * other.e[0] + self.e[1] * other.e[1] + self.e[2] * other.e[2];
    }

    #[inline]
    pub fn cross(self, other: Vec3) -> Vec3 {
        return Vec3 {
            e: [
                self.e[1] * other.e[2] - self.e[2] * other.e[1],
                self.e[2] * other.e[0] - self.e[0] * other.e[2],
                self.e[0] * other.e[1] - self.e[1] * other.e[0],
            ]
        }
    }

    #[inline]
    pub fn unit_vector(v: Vec3) -> Vec3 {
        return v / v.length();
    }

    #[inline]
    pub fn random() -> Vec3 {
        return Vec3 {
            e: [rtweekend::random_double(),
            rtweekend::random_double(),
            rtweekend::random_double()]
        }
    }

    #[inline]
    pub fn random_range(min: f32, max: f32) -> Vec3 {
        return Vec3 {
            e: [rtweekend::random_double_range(min, max),
                rtweekend::random_double_range(min, max),
                rtweekend::random_double_range(min, max)]
        }
    }

    #[inline]
    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        return Vec3::unit_vector(Vec3::random_in_unit_sphere());
    }

    pub fn near_zero(&self) -> bool {
        const s: f32 = (1 / 10_i32.pow(8)) as f32;
        return ((self.e[0]).abs() < s) && ((self.e[1]).abs() < s) && ((self.e[2]).abs() < s);
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        return (*v) - 2.0 * v.dot(*n) * (*n);
    }

    pub fn change(&mut self, other: Vec3) {
        self.e[0] = other.x();
        self.e[1] = other.y();
        self.e[2] = other.z();
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;
    #[inline]
    fn add(self, _other: Vec3) -> Self::Output {
        return Vec3 {
            e: [self.e[0] + _other.e[0],
                self.e[1] + _other.e[1],
                self.e[2] + _other.e[2],
            ]
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;
    #[inline]
    fn sub(self, _other: Vec3) -> Self::Output {
        return Vec3 {
            e: [self.e[0] - _other.e[0],
                self.e[1] - _other.e[1],
                self.e[2] - _other.e[2],
            ]
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, _other: Vec3) -> Vec3 {
        return Vec3 {
            e: [self.e[0] * _other.e[0],
                self.e[1] * _other.e[1],
                self.e[2] * _other.e[2],
            ]
        }
    }
}

impl ops::Div for Vec3 {
    type Output = Self;
    #[inline]
    fn div(self, _other: Vec3) -> Vec3 {
        return Vec3 {
            e: [self.e[0] / _other.e[0],
                self.e[1] / _other.e[1],
                self.e[2] / _other.e[2],
            ]
        }
    }
}

// vector / scalar
impl ops::Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, _other: f32) -> Self::Output {
        return Vec3 {
            e: [self.e[0] / _other,
                self.e[1] / _other,
                self.e[2] / _other,
            ]
        }
    }
}


// scalar * vector
impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    #[inline]
    fn mul(self, mut _other: Vec3) -> Self::Output {
        _other.e[0] = _other.e[0] * self;
        _other.e[1] = _other.e[1] * self;
        _other.e[2] = _other.e[2] * self;
        return _other;
    }
}

// impl std::ops::Div<Vec3> for f32 {
//     type Output = Vec3;
//
//     #[inline]
//     fn div(self, mut _other: Vec3) -> Self::Output {
//         _other.e[0] = _other.e[0] / self;
//         _other.e[1] = _other.e[1] / self;
//         _other.e[2] = _other.e[2] / self;
//         return _other;
//     }
// }

pub type Point3 = Vec3;
pub type Color = Vec3;