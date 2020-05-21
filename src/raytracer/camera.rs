use super::vector::Vector;

pub struct Camera {
    origin: Vector,
    size: (usize, usize)
}

impl Camera {
    pub fn new(size: (usize, usize)) -> Self {
        Camera {
            origin: Vector::new(0.0, 0.0, -10.0),
            size
        }
    }

    pub fn get_ray(&self, x: usize, y: usize) -> (Vector, Vector) {
        let max_len = usize::max(self.size.0, self.size.1) as f64 / 2.0;
        let direction = Vector {
            x: (x as f64 - (self.size.0 as f64 / 2.0)) / max_len,
            y: (y as f64 - (self.size.1 as f64 / 2.0))  / max_len,
            z: 1.5 // higher -> lower fov (less fisheye)
        };
        (self.origin, direction.normalized())
    }
}