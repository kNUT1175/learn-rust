fn main() {
    let mut count = 0;

    let result = loop {
        count += 1;

        if count == 10 {
            println!("{count}"); // this is interesting
            break count * 2; // similar to returning before we can see this just works
        }
    };

    println!("The result is {result}");
}
