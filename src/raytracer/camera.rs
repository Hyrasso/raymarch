use super::vector::Vector;

pub struct Camera {
    pub origin: Vector,
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

    pub fn get_rays(&self, x: usize, y: usize, n_samples: usize) -> Vec<(Vector, Vector)> {
        let max_len = usize::max(self.size.0, self.size.1) as f64 / 2.0;
        let mut res = Vec::new();
        let dpos5 = [
            Vector::new(0.0, 0.0, 0.0),
            Vector::new(0.5, 0.5, 0.0),
            Vector::new(0.5, -0.5, 0.0),
            Vector::new(-0.5, -0.5, 0.0),
            Vector::new(-0.5, 0.5, 0.0),
        ];
        for dpos in dpos5.iter().take(n_samples) {
            let direction = Vector {
                x: (x as f64 + dpos.x - (self.size.0 as f64 / 2.0)) / max_len,
                y: (y as f64 + dpos.y - (self.size.1 as f64 / 2.0))  / max_len,
                z: 1.5 // higher -> lower fov (less fisheye)
            };
            res.push((self.origin, direction.normalized()))
        }
        res
    }
}