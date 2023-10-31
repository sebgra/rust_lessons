fn main() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);

    println!("v contains : {:?}", v);

    let vector = vec![1, 2, 3, 4, 5, 6];

    let third : &i32 = &vector[2];

    println!("The third element is : {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // Iterating over vector

    let mut vector_to_iterate = vec![1, 2, 3, 4, 5, 6];


    for i in &mut vector_to_iterate{

        println!("The current value is : {i}");

        *i += 50;

        println!("The upgraded pointer to value is : {i}");
    }

    // Use vectors to store different types

    enum SpreadSheetCell{
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![SpreadSheetCell::Int(3), SpreadSheetCell::Text(String::from("blue")), SpreadSheetCell::Float(10.12)];

    
}
