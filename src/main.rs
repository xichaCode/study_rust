mod example;
mod structobj;

use std::fmt;
use std::fmt::{Arguments, Formatter, write};
use std::io::{IoSlice, Write};
use crate::example::{Parse, Rectangle};
use crate::structobj::{Add,Complex};

struct  BufBuilder {
    buf: Vec<u8>
}

impl BufBuilder {
    pub fn new() -> Self {
        Self {
            buf: Vec::with_capacity(1024),
        }
    }
}

impl fmt::Debug for BufBuilder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.buf))
    }
}

impl Write for BufBuilder {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buf.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}


fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels ." ,rect1.area());
    println!("The rectangle has a nonzero width; it is {},", rect1.width);

    let mut buf = BufBuilder::new();
    buf.write_all(b"Hello world !").unwrap();
    println!("{:?}",buf);
    println!("reslut: {}" ,u8::parse("255 hello world"));

    let c1 =Complex::new(1.0, 1f64);
    let c2= Complex::new(2 as f64, 3.0);
    println!("{:?}",c2+c1)
}
