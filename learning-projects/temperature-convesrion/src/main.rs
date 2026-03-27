use std::io;

static MULT: f64 = 5.0/9.0;

fn main() {
    println!("Temperature Converter");
    println!("Enter your temperature in Fahrenheit:");

    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let temp: f64 = temp
        .trim()
        .parse()
        .expect("Please type a number!");


    println!("You entered : {0} degrees fahrenheit", temp);
    let x = (temp-32.0) * MULT;
    println!("Which is {rounded} degrees celsius", rounded = (x * 1000.0).round() / 1000.0);

}
