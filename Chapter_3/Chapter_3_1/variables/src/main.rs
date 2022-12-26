fn main() {
    let mut x = 5;
    println!("The value of x is : {x}");

    x = 6;
    println!("The value of x is : {x}");

    // Shadowing

    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is : {y}");
    }

    println!("The value of y is : {y}");

    // Part to comment to let the script running to end

    let mut spaces = "   ";
    println!("The value of spaces is : {spaces}");

    spaces = spaces.len();

    println!("The value of spaces is : {spaces}");
}
