use super::color::Color;
use super::vector::Vector;
use super::material::Material;

pub struct Intersection {
    color: Color<u8>,
    point: Vector,
    normal: Vector,
    pub material: Material
}