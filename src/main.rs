use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

mod raytracer;
use raytracer::scene::Scene;
use raytracer::camera::Camera;

fn main() {
    let ratio = 2;
    let height = 600;
    let width = height * ratio;
    let mut scene: Scene = Scene::new();
    scene.set_camera(Camera::new((width, height)));
    scene.debug();
    
    println!("Start rendering");

    let buffer = scene.render(width, height);

    println!("Render complete");

    let path = Path::new(r"render.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width as u32, height as u32);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&buffer).unwrap(); // Save
}
