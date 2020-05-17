use super::color::Color;

pub struct Scene {

}

impl Scene {
    pub fn render(&self, width: usize, height: usize) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(width * height * 3);
        for y in 0..height {
            for x in 0..width {
                let color = self.get_pixel(x as f64 / width as f64, y as f64 / height as f64);
                buffer.push(color.red);
                buffer.push(color.green);
                buffer.push(color.blue);
            }
        }
        buffer
    }

    fn get_pixel(&self, x: f64, y: f64) -> Color<u8> {
        Color {
            red: 0,
            green: 0,
            blue: 0
        }
    }
}