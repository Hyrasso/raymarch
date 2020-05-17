
#[derive(Debug)]
pub struct Color<T> {
    pub red: T,
    pub green: T,
    pub blue: T
}

impl Color<u8> {
    pub fn new() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}