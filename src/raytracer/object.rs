use super::color::Color;
use super::vector::{Vector, UNIT_X, UNIT_Y, UNIT_Z};
use super::material::Material;
use std::boxed;

const DERIVATIVE_EPSILON: f64 = 1e-4;

pub trait Object {
    fn distance(&self, point: Vector) -> f64;
    fn normal(&self, point: Vector) -> Vector {
        // distance function derivative
        let dd = Vector {
            x: (self.distance(point + UNIT_X * DERIVATIVE_EPSILON)) - self.distance(point - UNIT_X * DERIVATIVE_EPSILON),
            y: self.distance(point + UNIT_Y * DERIVATIVE_EPSILON) - self.distance(point - UNIT_Y * DERIVATIVE_EPSILON),
            z: self.distance(point + UNIT_Z * DERIVATIVE_EPSILON) - self.distance(point - UNIT_Z * DERIVATIVE_EPSILON)
        };
        dd.normalized()
    }
    fn get_material(&self, point: Vector) -> Material;
}

pub struct Sphere {
    center: Vector,
    radius: f64,
    material: Material
}

impl Sphere {
    pub fn new(center: Vector, radius: f64, color: Color<u8>) -> Self {
        Sphere {
            center,
            radius,
            material: Material { color: color.into() }
        }
    }
}

impl Object for Sphere {
    fn distance(&self, point: Vector) -> f64 {
        (point - self.center).norm() - self.radius
    }

    fn get_material(&self, _point: Vector) -> Material {
        self.material
    }

    // fn normal(&self, point: Vector) -> Vector {
    //     (point - self.center).normalized()
    // }
}

pub struct Box {
    center: Vector,
    size: Vector,
    material: Material
}

impl Box {
    pub fn new(center: Vector, size: Vector, color: Color<u8>) -> Self {
        Box {
            center,
            size,
            material: Material::default_color(color.into())
        }
    }
}

impl Object for Box {
    fn distance(&self, point: Vector) -> f64 {
        let q: Vector = (point - self.center).abs() - self.size;
        // https://iquilezles.org/www/articles/distfunctions/distfunctions.htm
        q.max(0.0).norm() + q.x.max(q.y.max(q.z)).min(0.0)
    }

    fn get_material(&self, point: Vector) -> Material {
        self.material
    }
}

pub struct BlendObjects {
    pub objects: Vec<boxed::Box<Object>>,
    pub smooth_coef: f64
}

impl Object for BlendObjects {
    fn distance(&self, point: Vector) -> f64 {
        // exp smooth min https://www.iquilezles.org/www/articles/smin/smin.htm
        let res: f64 = self.objects.iter().map(|o| o.distance(point)).map(|d| f64::powf(2.0, -self.smooth_coef * d)).sum();
        -res.log2() / self.smooth_coef
    }

    fn get_material(&self, point: Vector) -> Material {
        // todo blend materials
        self.objects[0].get_material(point)
    }
}