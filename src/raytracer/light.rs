use super::vector::Vector;

pub trait Light {
    fn get_direction(&self, point: Vector) -> Vector;
}

pub struct DirectionalLight {
    direction: Vector
}

impl DirectionalLight {
    pub fn new(direction: Vector) -> Self {
        DirectionalLight {
            direction: direction.normalized()
        }
    }
}

impl Light for DirectionalLight {
    fn get_direction(&self, _point: Vector) -> Vector {
        self.direction * -1.0
    }
}