fn main() { //we cannot borrow a mutable twice

    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
    //second example

    let mut s2 = String::from("hello");

    change(&mut s2);

    println!("{}", s2);


}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()

}



fn change(some_string: &mut String) {
    some_string.push_str(", world");
} //mutable string so the string can be changed

fn calculate_length(some_string: &String) -> usize {
    some_string.len()
}
