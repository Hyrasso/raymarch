use super::vector::Vector;

#[derive(Copy, Clone)]
pub struct Material {
    pub color: Vector,
    pub reflection_coeff: f64,
    pub specular_coeff: f64,
    pub specular_power: f64,
}

impl Material {
    pub fn debug() -> Self {
        Material::default_color(Vector::new(1.0, 0.0, 1.0))
    }

    pub fn default_color(color: Vector) -> Self {
        Material {
            color,
            reflection_coeff: 0.0,
            specular_coeff: 0.8,
            specular_power: 20.0
        }
    }
}