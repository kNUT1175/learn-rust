use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}\n");

    //math with types

    let sum = 5+10;
    println!("The value of 5 + 10 = {sum}");

    let difference = 95.5 -4.3;
    println!("The value of 95.5 - 4.3 = {difference}");

    let product = 4 * 30;
    println!("The value of product is {product}");

    let product = 4 as f64 * 30.2; // also can be "4f64"
    println!("The value of product is {product}");

    let quotient = 56.7 / 32.2;
    println!("The value of quotient is {quotient}");

    let truncated = -4 / 3;
    println!("The value of truncated is {truncated}");

    let remainder = 43 % 3;
    println!("The value of remainder is {remainder}");

    let c = true;
    let f: bool = false;
    println!("The value of c is {c}");
    println!("The value of f is {f}");

    let z: char = 'x';
    println!("The value of z is {z}");


    //TUPLES
    let tup = (500, 600, 4.2, 'a');

    println!("The number in the second spot is {x}", x = tup.1);

    //ARRAYS
    let arr = [1, 2, 3, 4, 5];

    let two = arr[1];
    println!("The value of two is {two}");

    let a: [i32; 5] = [1, 2, 3, 4, 5]; //the type is given

    let b = [3; 5]; // type inference, all numbers are 3 (i32)

    println!("{x} + {y}\n",x = b[0], y = b[1]);

    println!("Please enter an array index. (0-4)");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of index {index} element is {element}");





}
