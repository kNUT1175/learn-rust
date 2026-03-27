use std::io;

fn main() {
    println!("Hello, world!");
    println!("Enter your message: ");

    let mut msg = String::new();


    io::stdin()
        .read_line(&mut msg)
        .expect("Failed to read line");

    // msg = msg.replace("ng", "ify");


    another_function(msg);
    println!("The number is {x}", x = five());

    println!("The answer is {y}",y = add_one(2));
}

fn another_function(x: String){
    println!("Another function printed {x}");
}

fn add_one(x: i32) -> i32{
    x + 1
}

fn five() -> i32{
    5
}
