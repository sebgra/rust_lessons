fn main() {
    
    let x = 2.0; // f64
    println!("The value of x is : {x}");

    let y : f32 = 3.0; // f32

    println!("The value of y is : {y}");

    // addition
    let sum = 5 + 10;
    // substraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product =  4*30;
    // division 
    let quotient = 56.7 / 32.2;
    let truncated = -5/3;

    //remainder
    let remainder = 43%5;

    // Boleans

    let t = true;

    let f: bool = false;

    let c = 'z';
    let z:  char = 'Z';

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is : {y}");
    
    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    println!("The value of five_hundred is : {five_hundred}");
    println!("The value of six_point_four is : {six_point_four}");

    // Arra type 
    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];

    let b: [i32; 5] = [1, 2, 3, 4, 5];

    println!("The value of b is : {:?}", b);

    let c = [3; 5];

    println!("The value of c is : {:?}", c);

    let first = a[0];

    let second = a[1];

    println!("The value of first is : {first}");
    println!("The value of second is : {second}");
}
