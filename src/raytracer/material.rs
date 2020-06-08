use super::vector::Vector;

#[derive(Copy, Clone)]
pub struct Material {
    pub color: Vector
}

impl Material {
    pub fn debug() -> Self {
        Material {
            color: Vector::new(1.0, 0.0, 1.0)
        }
    }

    pub fn default_color(color: Vector) -> Self {
        Material {
            color
        }
    }
}