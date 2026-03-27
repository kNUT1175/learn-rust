#[derive(Debug)]
enum UsState{
    California,
    Alaska,
    Washington,
    Montana,
    Oregon,
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin{
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x{
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn eval_dice(roll:i32){
    match roll{
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(roll),
        // _ => reroll()
        //the pattern above is for the catch-all if we don't want to use it
        // _ => ()
        //here we use an empty tuple to run nothing and use nothing
    }
}

fn add_fancy_hat(){}
fn remove_fancy_hat(){}
fn move_player(num_spaces:i32){
    println!("You moved {} spaces", num_spaces);
}

fn main() {
   let _some_number = Some(5);
    let _some_char = Some("e");

    let _absent_number: Option<i32> = None;

    value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let roll = 9;
    eval_dice(roll);


    //both sections below do the same thing
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum number is {}", max),
        None => (),
    }

    if let Some(max) = config_max {
        println!("The maximum number is {}", max);
    }

}
