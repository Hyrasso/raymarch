use std::ops::{Add, Mul, Sub, Div, Neg};
use std::convert::From;
use super::color::Color;

#[derive(Debug,Copy,Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector {
    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector {
            x,
            y,
            z
        }
    }

    pub fn dot(&self, other: &Vector) -> f64 {
        let product = Vector {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        };
        product.x + product.y + product.z
    }

    pub fn norm(&self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn normalized(&self) -> Vector {
        *self / self.norm()
    }
    // impl iter instead
    pub fn abs(&self) -> Vector {
        Vector {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs()
        }
    }

    pub fn max(&self, max: f64) -> Vector {
        Vector {
            x: self.x.max(max),
            y: self.y.max(max),
            z: self.z.max(max)
        }
    }

    pub fn powf(&self, n: f64) -> Vector {
        Vector {
            x: self.x.powf(n),
            y: self.y.powf(n),
            z: self.z.powf(n)
        }
    }
}

impl Add for Vector {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl Sub for Vector {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Self;
    
    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl Neg for Vector {
    type Output = Self;
    
    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

impl Div<f64> for Vector {
    type Output = Self;
    
    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }
    }
}

impl From<Color<u8>> for Vector {
    fn from(color: Color<u8>) -> Self {
        Vector {
            x: color.red as f64 / 255.0,
            y: color.green as f64 / 255.0,
            z: color.blue as f64 / 255.0,
        }
    }
}

pub const UNIT: Vector = Vector{x: 1.0, y: 1.0, z: 1.0};
pub const UNIT_X: Vector = Vector{x: 1.0, y: 0.0, z: 0.0};
pub const UNIT_Y: Vector = Vector{x: 0.0, y: 1.0, z: 0.0};
pub const UNIT_Z: Vector = Vector{x: 0.0, y: 0.0, z: 1.0};