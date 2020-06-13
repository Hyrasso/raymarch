use super::vector::Vector;

pub trait Light {
    fn get_direction(&self, point: Vector) -> Vector;
    fn get_color(&self) -> Vector;
}

pub struct DirectionalLight {
    direction: Vector,
    color: Vector
}

impl DirectionalLight {
    pub fn new(direction: Vector, color: Vector) -> Self {
        DirectionalLight {
            direction: direction.normalized(),
            color
        }
    }
}

impl Light for DirectionalLight {
    fn get_direction(&self, _point: Vector) -> Vector {
        self.direction * -1.0
    }

    fn get_color(&self) -> Vector {
        self.color
    }
}