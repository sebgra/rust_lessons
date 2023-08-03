enum IpAddrKind{ // Create enum structure
    v4(String),
    v6(String),
}

let home = IpAddrKind::v4(String::from("127.0.0.1")); // Create instance of enum
let loopback = IpAddrKind::v6(String::from("::1")); // Create instance of enum

enum Message{

    Quit, 
    Move{x : u32, y : u32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message{ // Implement method for enum
    fn call(&self){
        // method body
    }
}

let m = Message::Write(String::from("hello")); // Create instance of enum
m.call(); // Call method for enum


enum Option<T>{
    Some(T),
    None,
}

let some_number = Some(5);
let some_string = Some("e");

let absent: Option<i32> = None; // Must specify type if None

fn main() {
    
}
