use std::ops::{Add, Mul, Sub, Div};

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
