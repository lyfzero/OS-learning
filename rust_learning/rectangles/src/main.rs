#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size}
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // 元组重构
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area1(rect1)
    );

    // 结构体重构
    let rect2 = Rectangle { width: 30, height: 50};
    println!(
        "The area of the rectangle is {} square pixels.",
        area2(&rect2)
    );

    // 派生Debug trait，用调试格式打印rectangle
    println!("rect2 is {:#?}", rect2);

    // 方法
    println!("The area of the rectangle is {} square pixels.",
        rect2.area()
    );
    let rect3 = Rectangle {width:59, height: 54};

    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));

    // 关联函数
    let sq = Rectangle::square(3);
    println!("sq is {:#?}", sq);
}
fn area(width: u32, height: u32) -> u32 {
    width * height
}
fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
fn area2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}