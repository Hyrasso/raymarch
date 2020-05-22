use std::convert::From;
use super::vector::Vector;

#[derive(Debug,Copy, Clone)]
pub struct Color<T> {
    pub red: T,
    pub green: T,
    pub blue: T
}

impl Color<u8> {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Color {
            red,
            green,
            blue
        }
    }

    pub fn debug() -> Self {
        Self {
            red: 255,
            green: 0,
            blue: 255,
        }
    }
}

impl From<Vector> for Color<u8> {
    fn from(vec_color: Vector) -> Self {
        let color = vec_color * 255.0;
        
        Color {
            red: color.x.min(255.0).max(0.0) as u8,
            green: color.y.min(255.0).max(0.0) as u8,
            blue: color.z.min(255.0).max(0.0) as u8,
        }
    }
}

pub const RED: Color<u8> = Color {
    red: 255,
    green: 0,
    blue: 0,
};

pub const GREEN: Color<u8> = Color {
    red: 0,
    green: 255,
    blue: 0,
};

pub const BLUE: Color<u8> = Color {
    red: 0,
    green: 0,
    blue: 255,
};

pub const BLACK: Color<u8> = Color {
    red: 0,
    green: 0,
    blue: 0,
};