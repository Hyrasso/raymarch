use crate::raytracer::color::WHITE;
use crate::raytracer::vector::_UNIT;

use super::camera::Camera;
use super::color::{BLACK, Color};
use super::light::Light;
use super::vector::Vector;
use super::object::{Sphere, Box as OBox, BlendObjects, Object};
use super::material::Material;
use super::light::DirectionalLight;

pub struct Scene {
    background_color: Vector,
    camera: Camera,
    objects: Vec<Box<dyn Object>>,
    lights: Vec<Box<dyn Light>>,
    epsilon: f64,
    max_depth: i64,
    max_distance: f64
}

impl Scene {
    pub fn new() -> Self {
        Self {
            background_color: BLACK.into(),
            camera: Camera::new((400, 400)),
            objects: Vec::new(),
            lights: Vec::new(),
            epsilon: 1e-3,
            max_depth: 5,
            max_distance: 100.0
        }
    }

    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    pub fn add_object(&mut self, object: Box<dyn Object>) {
        self.objects.push(object);
    }
    
    pub fn add_light(&mut self, light: Box<dyn Light>) {
        self.lights.push(light);
    }

    pub fn render(&self, width: usize, height: usize) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(width * height * 3);
        for y in 0..height {
            for x in 0..width {
                let colorf = self.compute_color_at(x, y);
                // gamma correction
                let colorf = colorf.powf(1.0 / 2.2);
                let color: Color<u8> = colorf.into();
                buffer.push(color.red);
                buffer.push(color.green);
                buffer.push(color.blue);
                
            }
        }
        buffer
    }

    fn compute_color_at(&self, x: usize, y: usize) -> Vector {
        let rays = self.camera.get_rays(x, y, 5);
        // let dso = [Vector::new(0.0, 0.0, 0.0);4];
        let su: Vector = rays.iter().map(|(origin, direction)| -> Vector {
            self.cast_ray(*origin, *direction, self.max_depth).unwrap_or(self.background_color)
        }).fold(Vector::new(0.0, 0.0, 0.0), |acc, b| acc + b); // sum
        su / 5.0
    }

    fn cast_ray(&self, origin: Vector, direction: Vector, depth: i64) -> Option<Vector> {
        // println!("{:?}, {:?}", origin, direction);
        if depth <= 0 {
            return None;
        }
        let mut point = origin;
        let mut closest_distance = 1e9;
        for _step in 0..1_000 {
            if let Some(closest) = self.objects.iter().min_by(|a, b| a.distance(point).partial_cmp(&b.distance(point)).unwrap_or(std::cmp::Ordering::Equal)) { // very wrong to return equal, if we have to unwrap one of the distance if probably NaN or infinty
                let distance = closest.distance(point);
                if distance < self.epsilon {
                    // println!("Hit");
                    let material = closest.get_material(point);
                    let normal = closest.normal(point);
                    // correct hit position by epsilon to make sure were outside the object
                    let hit_position = point + direction.normalized() * (self.epsilon - distance);
                    // lighting
                    let mut color = Vector::zero();
                    for light in self.lights.iter() {
                        let towards_light = light.get_direction(hit_position);
                        // cast shadow
                        let offset_shadow_point = hit_position + towards_light * 1.0 / towards_light.dot(&normal).abs();
                        // The light could be between the objects
                        // TODO: check if the hit happens closer than the distance to the light
                        if self.cast_ray(offset_shadow_point, towards_light, 1.min(depth - 1)).is_some() {
                            // println!("Under shadow");
                            continue;
                        }
                        // N.L diffuse
                        let ambient: Vector = material.color * towards_light.dot(&normal).max(0.0);
                        // let ambient = Vector::zero();
                        // specular
                        // let light_direction = towards_light * -1.0;
                        let reflection: Vector = normal * 2.0 * (normal.dot(&towards_light)) - towards_light;
                        // point - origin : towards camera
                        let spec = (self.camera.origin - hit_position).normalized().dot(&reflection).max(0.0).powf(material.specular_power);
                        let specular: Vector = material.color * spec;
                        // let specular = Vector::zero();
                        color = color + light.get_color().mul(ambient + specular * material.specular_coeff);
                    }
                    if material.reflection_coeff <= 0.0 {
                        return Some(color)
                    }
                    let towards_origin = (origin - hit_position).normalized();
                    let reflection: Vector = (normal * 2.0 * (normal.dot(&towards_origin)) - towards_origin).normalized();
                    // todo compute actual distance cross product reflection and normal
                    let offset_reflection_point = hit_position + reflection * 2.0 * self.epsilon;
                    if let Some(reflection) = self.cast_ray(offset_reflection_point, reflection, depth - 1) {
                        return Some(color * (1.0 - material.reflection_coeff) + reflection * material.reflection_coeff);
                    } else {
                        return Some(color);
                    }
                }
                if distance > self.max_distance {
                    // println!("Going too far away {:?} after {:?} steps", distance, step);
                    break;
                }
                // for halo effect
                closest_distance = distance.min(closest_distance);
                // march
                point = point + direction * distance;
            } else {
                // no closes objects, object list is empty (is it the only possible way we end up here?)
                break;
            }
        }
        // adds a lot of light to the scene
        // allowing glow on reflection as well adds a lot of light to the scene
        let glow_distance = 0.2;
        if closest_distance < glow_distance && depth >= self.max_depth {
            // could have some cool rainbow/chromatic aberation effects by shifting some colors depending on the distance
            let glow = _UNIT * 0.05 * f64::exp(-7.0 * closest_distance / glow_distance);
            return Some(glow);
        }
        None
    }

    /// Setup the debug scene
    pub fn debug(&mut self) {
        // scene.add_object(Object::Sphere(Vector::new(0.0, 0.0, 3.0), 4.0));
        let scb = Box::new(BlendObjects {
            objects: vec![
                Box::new(Sphere {
                    center: Vector::new(1.2, -1.0, -0.2),
                    radius: 1.0,
                    material: Material {
                        color: Vector::new(0.1, 0.1, 1.0),
                        ..Material::debug()
                    }
                }),
                Box::new(OBox::new(
                    Vector::new(2.0, 0.0, 0.0),
                    Vector::new(1.0, 1.0, 1.0),
                    Color::debug().into()
                ))
            ],
            smooth_coef: 16.0
        });
        self.add_object(scb);
        self.add_object(Box::new(Sphere {
            center: Vector::new(-2.0, 0.0, 0.0),
            radius: 2.0,
            material: Material {
                color: Vector::new(1.0, 10.0 / 255.0, 10.0 / 255.0),
                reflection_coeff: 0.2,
                specular_coeff: 0.95,
                specular_power: 35.0,
                ..Material::debug()
            }
        }));
        
        // floor box
        self.add_object(Box::new(OBox {
                center: Vector::new(0.0, 10.0, 0.0),
                size: Vector::new(10.0, 8.0, 10.0),
                material: Material {
                    reflection_coeff: 0.95,
                    ..Material::default_color(Vector::from(WHITE) * 0.1)
                }
        }));
    
        // scene.add_light(Box::new(DirectionalLight::new(Vector::new(-1.0, -1.0, 0.0))));
        self.add_light(Box::new(DirectionalLight::new(Vector::new(1.0, 0.1, 0.5), _UNIT * 0.4)));
        self.add_light(Box::new(DirectionalLight::new(Vector::new(-0.1, 1.0, 1.0), Vector::new(0.4, 0.1, 0.5) * 0.1)));
        // box behind the camera
        // self.add_object(Box::new(OBox::new(Vector::new(0.0, 0.0, -11.1), Vector::new(1.0, 1.0, 1.0), Color::debug())));
    }
}
