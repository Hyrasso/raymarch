use super::color::Color;
use super::vector::Vector;
use super::camera::Camera;
use super::object::Object;

pub struct Scene {
    background_color: Color<u8>,
    camera: Camera,
    objects: Vec<Box<Object>>,
    epsilon: f64,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            background_color: Color {
                red: 60,
                green: 60,
                blue: 60,
            },
            camera: Camera::new((400, 400)),
            objects: Vec::new(),
            epsilon: 1e-3
        }
    }

    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    pub fn add_object(&mut self, object: Box<Object>) {
        self.objects.push(object);
    }

    pub fn render(&self, width: usize, height: usize) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(width * height * 3);
        for y in 0..height {
            for x in 0..width {
                if let Some(color) = self.get_pixel(x, y) {
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

    fn get_pixel(&self, x: usize, y: usize) -> Option<Color<u8>> {
        let (origin , direction) = self.camera.get_ray(x, y);
        self.cast_ray(origin, direction)
    }

    fn cast_ray(&self, origin: Vector, direction: Vector) -> Option<Color<u8>> {
        // println!("{:?}, {:?}", origin, direction);
        let mut pres_dist = 1e9;
        let mut point = origin;
        for step in 0..1_000 {
            if let Some(closest) = self.objects.iter().min_by(|a, b| a.distance(point).partial_cmp(&b.distance(point)).unwrap()) {
                let distance = closest.distance(point);
                if distance < self.epsilon {
                    // println!("Hit");
                    let material = closest.get_material(origin);
                    // color
                    // reflection
                    // lighting
                    return Some(Color::from(material.color));
                }
                if distance > pres_dist {
                    // println!("Going too far away {:?} after {:?} steps", distance, step);
                    return None;
                }
                point = point + direction * distance;
                pres_dist = distance;
            } else {
                println!("no objects");
                return None;
            }
        }
        None
    }
}
