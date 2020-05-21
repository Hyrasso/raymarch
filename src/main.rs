use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

mod raytracer;
use raytracer::scene::Scene;
use raytracer::vector::Vector;
use raytracer::object::Object;
use raytracer::camera::Camera;

fn main() {
    let mut scene: Scene = Scene::new();
    let width = 800;
    let height = 400;
    scene.set_camera(Camera::new((width, height)));
    scene.add_object(Object::Sphere(Vector::new(0.0, 0.0, 0.0), 2.0));
    // scene.add_object(Object::Sphere(Vector::new(0.0, 0.0, 3.0), 4.0));
    scene.add_object(Object::Sphere(Vector::new(-3.0, -2.0, 0.0), 1.0));
    scene.add_object(Object::Sphere(Vector::new(5.0, 0.0, 0.0), 1.0));
    let buffer = scene.render(width, height);

    let path = Path::new(r"render.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width as u32, height as u32);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&buffer).unwrap(); // Save
}
