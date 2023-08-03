#[derive(Debug)] // Add Debug trait to Coin enum to allow for printing

enum UsState{ // Create enum structure that can be used to represent US states
    Alabama,
    Alaska,
    // ... etc
}

enum Coin{ // Create enum structure that can be used to represent coins but only represent one type
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // Add UsState enum as parameter to Quarter variant
}

fn value_in_cents(coin: Coin) -> u8{ // Create function that takes in Coin enum as parameter to return th associated value

    match coin{
        Coin::Penny => {
            println!("Lucky penny!"); // Multiple lines of code can be executed for each match arm
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state)=> {
            println!("State quarter from {:?}!", state); // Can use variables in match arms
            25},
    }
}

// Matching with Option<T>

fn plus_one(x: Option<i32>) -> Option<i32>{ // Create function that takes in Option<i32> as parameter to return Option<i32>

    match x{
        None => None, // Match None variant
        Some(i) => Some(i + 1), // Match Some variant and add 1 to value
    }
}

// The _ Placeholder
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}

let dice_roll = 9;
match dice_roll{
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other), // _ placeholder matches all other values of dice_roll
    // _ => reroll(),
    // _ => () // _ placeholder matches all other values of dice_roll but do nothing through empty tuple
}



fn main(){

    let tokenPenny = Coin::Penny;
    let tokenQuarter = Coin::Quarter(UsState::Alaska);
    let value = value_in_cents(tokenPenny);
    println!("The value of the coin is {} cents", value);
    let value = value_in_cents(tokenQuarter);
    println!("The value of the coin is {} cents", value);

    // Option part

    let five = Some(5);
    println!("The value of five is {:?}", five);
    let six = plus_one(five);
    println!("The value of six is {:?}", six);
    let none = plus_one(None);
    println!("The value of none is {:?}", none);

}
