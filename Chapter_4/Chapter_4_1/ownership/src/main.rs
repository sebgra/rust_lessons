fn main() {
    let mut s = String::from("hello");

    println!("The value of string is : {s}");

    // Modify string

    s.push_str(",world!");
    println!("{}", s); // This will print `hello, world!`

    let s1 = String::from("hello");
    let s2 = s1.clone(); // Need to call clone() method to bind previous variable to new variable

    println!("s1 = {}, s2 = {}", s1, s2);

    let x = 5;
    let y = x;

    println!("Value of x is: {}, and y is : {}", x, y); // clone is not needed because the size of an integer is already known

    takes_ownership(s);

    makes_copy(x);

    let s3 = gives_ownership();

    let s4 = String::from("hello");

    let s5 = takes_and_gives_back(s2);

    let (s6, len) = calculate_length(s4);

    println!("The length of '{}' is {}.", s6, len); 

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String){ //some_string comes into scope

    println!("{}", some_string);
} // Here some_string goes out of the scope, drop is called. The backing memory is freed.

fn makes_copy(some_integer: i32){// some_integer comes into the scope

    println!("{}", some_integer);
}// Here some_string goes out of the scope. Nothinng special happens.

fn gives_ownership() -> String{// gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
    // moves out to the calling
    // function
}

fn takes_and_gives_back(a_string: String) -> String{ // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}


fn calculate_length(s: String) -> (String, usize){

    let length = s.len();
    (s, length)
}