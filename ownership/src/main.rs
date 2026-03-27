fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // note: cloe is needed because rust would drop s1 otherwise
    //do not use clone() it's wack (causes more memory allocation)
    println!("{}", s1);
    println!("{}", s2);

    let mut s:String = String::from("hello");
    s.push_str(", world");
    println!("{s}");

    let s3 = String::from("hello");
    // let s4 = s3;

    println!("{}",s3); //this does not work anymore because of line above

    let x = 5;
    let y = x;

    println!("Y is: {y}");
    println!("X is: {x}"); // since ints are set in size this works

    let s4 = String::from("hello");

    takes_ownership(s4);

    // println!("s4: {s4}");
    //we can't use s4 since the function took ownership

    let z = 5;

    makes_copy(z);

    println!("z is: {z}");
    
    // we can still use z since it has the copy trait (int, bool, float, chat, tuple)

}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} //some_string goes out of scope and 'drop' is called, the backing memory is freed

fn makes_copy(some_integer: i32){
    println!("{}", some_integer);
} // some_integer goes out of scope, nothing special happens
