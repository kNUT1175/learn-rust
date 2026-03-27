enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}


fn main() {

    let v1 = vec![1, 2, 3]; // implicit type

    let mut v2 = Vec::new();

    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);

    // we can use pop as well

    let third: &i32 = &v2[2];
    println!("THe third element {third}.");

    let third: Option<&i32> = v2.get(2);
    match third{
        Some(third) => println!("The third element is {third}."),
        None => println!("There is no third element"),
    }

    // let does_not_exist = &v[100];
    // let does_not_exist = &v[100];

    let v3 = vec![100, 32, 57];


    for i in &v3{
        println!("{i}");
    }


    let mut v4 = vec![100, 32, 57];

    for i in &mut v4{
        *i += 50;
        println!("{i}")
    }

    let row:Vec<SpreadsheetCell> = vec![ // defined type is as shown
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    let mut _st:String = String::new();

    let data = "initial contents";
    let s = data.to_string();
    println!("{s}");


    let mut s = String::from("Foo");
    s.push_str("bar");

    println!("{s}");

    let mut s2 = String::from("Foo");
    let s3 = "bar";
    s2.push_str(s3);
    println!("{s3}"); // still works

    let mut s4 = String::from("lo");
    s4.push('l');
    println!("{s4}");

    let plusstr = String::from("Hello ");
    let plusstr2 = String::from("world!");
    let str3 = plusstr + &plusstr2; // plusststr has been moved now

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    //this is unwieldy and bad
    let s = s1 + "-" + &s2 + "-" + &s3;


    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}"); // does not take ownership
    println!("{s}");
    println!("{}{}{}", s1, s2, s3);

    for c in "alex".chars(){
        println!("{c}");
    }

    for b in "alex".bytes(){
        println!("{b}");
    }


















}
