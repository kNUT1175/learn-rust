fn main() {

    let s:String = String::from("hello");

    let slice = &s[0..2];
    let slice2 = &s[..2];

    let len = s.len();

    let slice3 = &s[3..len];
    let slice4 = &s[3..];

    let slice5 = &s[0..len];
    let slice6 = &s[..];

    println!("{} {}", slice, slice2);
    println!("{} {}", slice3, slice4);
    println!("{} {}", slice5, slice6);

    let my_string = String::from("hello world");

    let word = first_word(&my_string[..]);


}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
