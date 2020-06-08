use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

mod raytracer;
use raytracer::scene::Scene;
use raytracer::vector::Vector;
use raytracer::object::{Sphere, Box as OBox, BlendObjects};
use raytracer::camera::Camera;
use raytracer::color;
use raytracer::material::Material;
use raytracer::color::Color;
use raytracer::light::DirectionalLight;

fn main() {
    let mut scene: Scene = Scene::new();
    let width = 800;
    let height = 400;
    scene.set_camera(Camera::new((width, height)));
    // scene.add_object(Object::Sphere(Vector::new(0.0, 0.0, 3.0), 4.0));
    let scb = Box::new(BlendObjects {
        objects: vec![
            Box::new(Sphere::new(Vector::new(1.0, -1.0, -0.2), 1.0, Color::new(10, 10, 255))),
            Box::new(OBox::new(
                Vector::new(2.0, 0.0, 0.0),
                Vector::new(1.0, 1.0, 1.0),
                Color::debug().into()
            ))
        ],
        smooth_coef: 16.0
    });
    scene.add_object(scb);
    scene.add_object(Box::new(Sphere::new(Vector::new(-2.0, 0.0, 0.0), 2.0, Color::new(255, 10, 10))));

    // scene.add_light(Box::new(DirectionalLight::new(Vector::new(-1.0, -1.0, 0.0))));
    scene.add_light(Box::new(DirectionalLight::new(Vector::new(1.0, 0.0, 0.5))));
    // scene.add_light(Box::new(DirectionalLight::new(Vector::new(0.0, -1.0, -0.2))));
    let buffer = scene.render(width, height);

    print!("Render complete");

    let path = Path::new(r"render.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width as u32, height as u32);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&buffer).unwrap(); // Save
}
