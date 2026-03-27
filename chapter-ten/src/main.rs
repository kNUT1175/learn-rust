
fn largest_fn<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point2 <X1, Y1>{
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point2 <X1, Y1>{
    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2 <X1, Y2>{
        Point2{x: self.x,
            y: other.y}
    }
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &U {
        &self.y
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


fn main() {

    let number_list = vec![34, 50, 25, 100, 65];

    println!("The largest number is {}", largest_fn(&number_list));

    let char_list = vec!['y', 'm', 'a', 'q'];

    println!("The largest char is {}", largest_fn(&char_list));

    let _integer = Point { x: 5, y: 10 };
    let _float = Point { x: 1.0, y: 4.0 };

    let _will_work = Point {x: 5.0, y: 10};

    let p = Point { x: 5, y: 10.4 };

    println!("p.x = {}, p.y = {}", p.x(), p.y());

    let p1 = Point { x: 5.5, y: 10.4 };

    println!("P1 distance from origin: {}", p1.distance_from_origin());

    let p2 = Point2 { x: "Hello", y: 'c' };
    let p1 = Point2 { x: 5, y: 10.4 };

    let p3 = p1.mixup(p2);

    println!("{} {}", p3.x, p3.y);





}
