#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

pub fn get_rectangle() {
    //创建Rectangle的实例
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };


}
//使用struct类型结构
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

