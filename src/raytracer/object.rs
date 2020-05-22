use super::color::Color;
use super::vector::Vector;
use super::material::Material;

pub trait Object {
    fn get_material(&self, point: Vector) -> Material;
    fn distance(&self, point: Vector) -> f64;
    fn normal(&self, point: Vector) -> Vector;
}

pub struct Sphere {
    center: Vector,
    radius: f64,
    material: Material
}

impl Sphere {
    pub fn new(center: Vector, radius: f64, color: Color<u8>) -> Self {
        Sphere {
            center,
            radius,
            material: Material { color: color.into() }
        }
    }
}

impl Object for Sphere {
    fn distance(&self, point: Vector) -> f64 {
        (point - self.center).norm() - self.radius
    }

    fn get_material(&self, _point: Vector) -> Material {
        self.material
    }

    fn normal(&self, point: Vector) -> Vector {
        (point - self.center).normalized()
    }
}
