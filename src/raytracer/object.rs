use super::color::Color;
use super::vector::Vector;
use super::material::Material;

pub enum Object {
    Sphere(Vector, f64)
}


impl Object {
    pub fn distance(&self, point: Vector) -> f64 {
        match self {
            Object::Sphere(center, radius) => (point - *center).norm() - radius
        }
    }
}
