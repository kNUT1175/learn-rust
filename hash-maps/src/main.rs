use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    //access data from hashmap
    let team_name = String::from("Blue");
    let _score = scores.get(&team_name).copied().unwrap_or(0);

    //iterate through the hashmap
    for (key, value) in &scores{
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color:");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    //field_name and field_value are invalid since they are strings and the map took ownership of them

    //overwriting a value in hashmap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    //inserting if there isn't already a value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50); //will add yellow, 50
    scores.entry(String::from("Blue")).or_insert(50); // will not add blue, 50

    println!("{:?}", scores);

    //updating based on old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
