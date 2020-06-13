use super::color::{Color, BLACK, WHITE};
use super::camera::Camera;
use super::light::Light;
use super::vector::Vector;
use super::object::{Sphere, Box as OBox, BlendObjects, InsideOutObject, Object};
use super::material::Material;
use super::light::DirectionalLight;

pub struct Scene {
    background_color: Color<u8>,
    camera: Camera,
    objects: Vec<Box<Object>>,
    lights: Vec<Box<Light>>,
    epsilon: f64,
    max_depth: i64,
    max_distance: f64
}

impl Scene {
    pub fn new() -> Self {
        Self {
            background_color: BLACK,
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
        self.cast_ray(origin, direction, self.max_depth).map(|color| color.powf(2.0).into())
    }

    fn cast_ray(&self, origin: Vector, direction: Vector, depth: i64) -> Option<Vector> {
        // println!("{:?}, {:?}", origin, direction);
        if depth <= 0 {
            return None;
        }
        let mut point = origin;
        let mut closest_distance = 1e9;
        for _step in 0..1_000 {
            if let Some(closest) = self.objects.iter().min_by(|a, b| a.distance(point).partial_cmp(&b.distance(point)).expect("NaN distance")) {
                let distance = closest.distance(point);
                if distance < self.epsilon {
                    // println!("Hit");
                    let material = closest.get_material(point);
                    let normal = closest.normal(point);
                    // correct hit position to be out the object by epsilon
                    let hit_position = point + direction.normalized() * (self.epsilon - distance);
                    // lighting
                    let mut color = Vector::zero();
                    for light in self.lights.iter() {
                        let towards_light = light.get_direction(hit_position);
                        // cast shadow
                        let offset_shadow_point = hit_position + towards_light * 1.0 / towards_light.dot(&normal).abs();
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
                        let spec = (origin - hit_position).normalized().dot(&reflection).max(0.0).powf(material.specular_power);
                        let specular: Vector = material.color * spec;
                        // let specular = Vector::zero();
                        color = color + ambient + specular * material.specular_coeff;
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
                point = point + direction * distance;
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

    pub fn debug(&mut self) {
        // scene.add_object(Object::Sphere(Vector::new(0.0, 0.0, 3.0), 4.0));
        let scb = Box::new(BlendObjects {
            objects: vec![
                Box::new(Sphere {
                    center: Vector::new(1.0, -1.0, -0.2),
                    radius: 1.0,
                    material: Material {
                        color: Vector::new(0.1, 0.1, 1.0),
                        reflection_coeff: 0.5,
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
        self.add_object(Box::new(Sphere::new(Vector::new(-2.0, 0.0, 0.0), 2.0, Color::new(255, 10, 10))));
        
        // floor box
        self.add_object(Box::new(OBox {
                center: Vector::new(0.0, 10.0, 0.0),
                size: Vector::new(10.0, 8.0, 10.0),
                material: Material {
                    reflection_coeff: 0.9,
                    ..Material::default_color(WHITE.into())
                }
        }));
    
        // scene.add_light(Box::new(DirectionalLight::new(Vector::new(-1.0, -1.0, 0.0))));
        self.add_light(Box::new(DirectionalLight::new(Vector::new(1.0, -0.1, 0.5), WHITE.into())));
        self.add_light(Box::new(DirectionalLight::new(Vector::new(-0.1, 1.0, 1.0), Vector::new(0.5, 0.2, 0.6))));
        // box behind the camera
        // self.add_object(Box::new(OBox::new(Vector::new(0.0, 0.0, -11.1), Vector::new(1.0, 1.0, 1.0), Color::debug())));
    }
}
