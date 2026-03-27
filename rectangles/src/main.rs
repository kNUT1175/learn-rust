#[derive(Debug)]

struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, within_rect: Rectangle) -> bool {
        self.width > within_rect.width && self.height > within_rect.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

}

impl Rectangle{  //&self is short for: self: &Self
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn set_width(&mut self, value:u32){
        self.width = value;
    }

    fn set_height(&mut self, value:u32){self.height = value;}

}

fn main() {

    // let width: u32 = 30;
    // let height: u32 = 50;

    // print!("The rectangle is {} square pixels.", area(width, height));

    // let rect1 = (30, 50);

    // println!("The area of the rectangle is {} square pixels.", area(rect1));

    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle{
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle{
        width: 60,
        height: 45,
    };

    let sq:Rectangle  = Rectangle::square(30);

    println!("The square has a width of {} and a height of {}.", sq.width, sq.height);

    // println!("The area of the rectangle is {} square pixels.", area(&rect1));

    println!("The area of the rectangle is {}", rect1.area());

    if rect1.width() {
        println!("The rectangle is width of {}", rect1.width);
    }

    println!("Rect1 is {:?}", rect1);
    println!("Rect1 is {:#?}", rect1);


    println!("Can rect2 fit inside rect1? {}", rect1.can_hold(rect2));
    println!("Can rect3 fit inside rect1? {}", rect1.can_hold(rect3));

}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area(rectangle: (u32, u32)) -> u32{
//     rectangle.0 * rectangle.1
// }
