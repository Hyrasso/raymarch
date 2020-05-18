use super::color::Color;
use super::object::Intersection;
use super::vector::Vector;

pub struct Scene {
    background_color: Color<u8>
}

impl Scene {
    pub fn new() -> Self {
        Self {
            background_color: Color {
                red: 60,
                green: 60,
                blue: 60,
            }
        }
    }

    pub fn render(&self, width: usize, height: usize) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(width * height * 3);
        for y in 0..height {
            for x in 0..width {
                if let Some(color) = self.get_pixel(x as f64 / width as f64, y as f64 / height as f64) {
                    buffer.push(color.red);
                    buffer.push(color.green);
                    buffer.push(color.blue);
                } else {
                    buffer.push(self.background_color.red);
                    buffer.push(self.background_color.green);
                    buffer.push(self.background_color.blue);
                }
                
            }
        }
        buffer
    }

    fn get_pixel(&self, x: f64, y: f64) -> Option<Color<u8>> {
        let origin = Vector::zero();
        let direction = Vector::zero();
        if let Some(intersection) = self.castRay(origin, direction) {
            Some(intersection.material.color)
        } else {
            None
        }
    }

    fn castRay(&self, origin: Vector, direction: Vector) -> Option<Intersection> {
        None
    }
}
