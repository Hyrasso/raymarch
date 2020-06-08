use super::color::{Color, BLACK};
use super::vector::Vector;
use super::camera::Camera;
use super::object::Object;
use super::light::Light;

pub struct Scene {
    background_color: Color<u8>,
    camera: Camera,
    objects: Vec<Box<Object>>,
    lights: Vec<Box<Light>>,
    epsilon: f64,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            background_color: BLACK,
            camera: Camera::new((400, 400)),
            objects: Vec::new(),
            lights: Vec::new(),
            epsilon: 1e-3
        }
    }

    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    pub fn add_object(&mut self, object: Box<Object>) {
        self.objects.push(object);
    }
    
    pub fn add_light(&mut self, light: Box<Light>) {
        self.lights.push(light);
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
        // gamma correction
        self.cast_ray(origin, direction).map(|color| color.powf(2.0).into())
    }

    fn cast_ray(&self, origin: Vector, direction: Vector) -> Option<Vector> {
        // println!("{:?}, {:?}", origin, direction);
        let mut pres_dist = 1e9;
        let mut point = origin;
        let mut closest_distance = 1e9;
        for _step in 0..1_000 {
            if let Some(closest) = self.objects.iter().min_by(|a, b| a.distance(point).partial_cmp(&b.distance(point)).expect("NaN distance")) {
                let distance = closest.distance(point);
                if distance < self.epsilon {
                    // println!("Hit");
                    let material = closest.get_material(point);
                    let normal = closest.normal(point);
                    // lighting
                    let mut color = Vector::zero();
                    for light in self.lights.iter() {
                        let towards_light = light.get_direction(point);
                        // N.L diffuse
                        let ambient: Vector = material.color * towards_light.dot(&normal).max(0.0);
                        // let ambient = Vector::zero();
                        // specular
                        // let light_direction = towards_light * -1.0;
                        let reflection: Vector = normal * 2.0 * (normal.dot(&towards_light)) - towards_light;
                        // point - origin : towards camera
                        let spec = (origin - point).normalized().dot(&reflection).max(0.0).powf(20.0);
                        let specular: Vector = material.color * spec;
                        // let specular = Vector::zero();
                        color = color + ambient + specular * 0.8;
                    }
                    return Some(color);
                    // if color.norm() < 0.1 {
                    //     return Some(color);
                    // }
                    // let towards_origin = (origin - point).normalized();
                    // let reflection: Vector = normal * 2.0 * (normal.dot(&towards_origin)) - towards_origin;
                    // if let Some(reflection) = self.cast_ray(point + reflection.normalized() * self.epsilon, reflection.normalized()) {
                    //     return Some(color * 0.8 + reflection * 0.2);
                    // } else {
                    //     return Some(color);
                    // }
                }
                if distance > 2.0 * pres_dist {
                    // println!("Going too far away {:?} after {:?} steps", distance, step);
                    break;
                }
                closest_distance = distance.min(closest_distance);
                point = point + direction * distance;
                pres_dist = distance;
            } else {
                break;
            }
        }
        let glow_distance = 0.5;
        if closest_distance < glow_distance {
            let glow = Vector::new(1.0, 1.0, 1.0) * (0.8 - closest_distance * 0.8 / glow_distance).powf(5.0);
            return Some(glow);
        }
        None
    }
}
