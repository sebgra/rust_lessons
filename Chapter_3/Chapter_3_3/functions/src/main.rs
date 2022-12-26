fn main() {
    println!("Hello, world!");
    another_function(5);
    print_labeled_measurment(5, 'h');

    let x = five();

    println!("The value of x is : {x}");

   let y =  plus_one(5);

   println!("The value of the number after increment is : {y}");

}

fn another_function(x: i32){
    println!("The value of x is: {x}");
}

fn print_labeled_measurment(value: i32, unit_label: char){

    println!("The measurment is: {value}{unit_label}");
}

fn five() -> i32{

    5 // No semi colon needed to return value
}

fn plus_one(x: i32) -> i32{

    x + 1 // No semi colon needed to return value

}
