mod redius;
mod image;
mod selfs;

use regex::Regex;
use std::str::FromStr;
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

pub trait Parse {
    fn parse(s: &str) -> Self;
}

impl Parse for u8 {
    fn parse(s: &str) -> Self {
        let re: Regex =Regex::new(r"^[0-9]+]").unwrap();
        if let Some(captures) = re.captures(s) {
            captures
                .get(0)
                .map_or(0, |s| s.as_str().parse().unwrap_or(0))
        } else {
            0
        }
    }
}
#[test]
fn parse_should_work() {
    assert_eq!(u8::parse("123abcd"), 123);
    assert_eq!(u8::parse("1234abcd"), 0);
    assert_eq!(u8::parse("abcd"), 0);
}