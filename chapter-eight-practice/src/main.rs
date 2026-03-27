use std::collections::HashMap;

fn main() {
    let mut vec = vec![95, 32, 66, 25, 5, 67, 74, 43, 11, 79, 77, 77, 32, 32, 5, 67, 87, 34, 56, 72, 87, 65, 34, 32];
    vec.sort();

    for item in &vec{
        print!("{item}, ");
    }

    println!("\n");

    let third = &vec[2];
    println!("The third element is {third}");

    let fourth = vec.get(3);
    match fourth {
        Some(fourth) => println!("The fourth element is {fourth}"),
        None => println!("You messed up"),
    }

    let median_num = (&vec.len()) / 2;

    let median_of_vec = &vec[median_num];

    println!("\nThe median is: {median_of_vec}");

    let mut hash_mode = HashMap::new();

    for items in &vec{
        let count =hash_mode.entry(items).or_insert(0);
        *count += 1;
    }


    println!("{hash_mode:?}");

    let mut max_val = 0;
    let mut max_key = None;
    for (k, v) in &hash_mode {
        if *v > max_val{
            max_key = Some(*k);
            max_val = *v;
        }
    }

    println!("\n");

    println!("mode = {}, count = {}", match max_key{ Some(k) => *k, None => 0, }, max_val);


    //pig latin


}
