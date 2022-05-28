mod example;
use std::fmt;
use std::io::Write;
use crate::example::Rectangle;

struct  BufBuilder {
    buf: Vec<u8>
}


fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels ." ,rect1.area());
    println!("The rectangle has a nonzero width; it is {},", rect1.width)
}
