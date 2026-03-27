use std::fs::File;
use std::io;
use std::fs;
use std::io::{ErrorKind, Read};
use std::net::IpAddr;

//this is an example of propagation, we allow the calling code to decide what to do if there is an error
//rather than handling it within the function
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hurdy.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username: String = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut username_file_result = File::open("hurdy.txt")?;
    let mut username: String = String::new();
    username_file_result.read_to_string(&mut username)?;
    Ok(username)

}

fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut username:String = String::new();
    File::open("hurdy.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn read_username_from_file_4() -> Result<String, io::Error> {
    fs::read_to_string("hurdy.txt")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess {value}
    }


    pub fn value(&self) -> i32 {
        self.value
    }

}



fn main() {

    let greeting_file_result = File::open("hello.txt");


    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
                }
            }
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        }
    };

    let _second_greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem: {:?}", error);
            })
        } else {
            panic!("Tried to create file but there was a problem: {:?}", error);
        }
    });

    //let _content = File::open("hello2.txt").unwrap();
    //use expect instead
    let _content = File::open("hello3.txt")
        .expect("hello3.txt should be included in this project");

    let home:IpAddr = "127.0.0.1"
        .parse()
        .unwrap();



}
