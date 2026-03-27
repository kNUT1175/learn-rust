use std::io;

fn main() {

    let mut n = String::new();

    println!("Please enter a number for the nth Fibonacci sequence:");

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: u32 = n
        .trim()
        .parse()
        .expect("Not a number!");

    println!();

    let mut prev: u32 = 0;
    let mut curr: u32 = 1;

    if n == 0{
        println!("Fibonacci sequence empty!");
    }

    for _i in 0..n-1 {
        let next = curr + prev;
        prev = curr;
        curr = next;
    }

    println!( "Nth Fibonacci is: {}", curr );


}
