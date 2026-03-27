fn main() {

    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

    println!("s1: {} \ns3: {}", s1, s3);


    let s4 = String::from("hello");
    let (s5, len) = calculate_length(s4);
    println!("The length of '{s5}' is {len}.");


}

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string
}

fn takes_and_gives_back(the_string: String) -> String {
    the_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
