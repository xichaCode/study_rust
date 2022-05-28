#[derive(Debug)]
pub struct Rectangle {
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
    pub fn width(&self) -> bool {
        self.width > 0
    }
}

impl Rectangle {
    pub fn areas(rectangle: &Rectangle) -> u32{
        rectangle.width * rectangle.height
    }
}