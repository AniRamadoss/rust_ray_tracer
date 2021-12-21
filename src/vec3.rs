use std::ops;

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

}

impl ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, _other: Vec3) -> Self::Output {
        return Vec3 {
            e: [self.e[0] + _other.e[0],
                self.e[1] + _other.e[1],
                self.e[2] + _other.e[2],
            ]
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;
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
    fn div(self, _other: Vec3) -> Vec3 {
        return Vec3 {
            e: [self.e[0] / _other.e[0],
                self.e[1] / _other.e[1],
                self.e[2] / _other.e[2],
            ]
        }
    }
}