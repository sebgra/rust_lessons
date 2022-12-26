fn main() {
    let number = 6;

    if number < 5{

        println!("Condition was true");
    }
    else{

        println!("Condition was false");
    }

    if number !=0{

        println!("number was something else than zero");
    }

    if number % 4 == 0{
        println!("Number is divisible by 4");
    }
    else if number % 3 ==0{
        println!("The number is divisible by 3");
    }
    else if number % 2 == 0{
        println!("The number is divisible by 2");
    }
    else{
        println!("The number is not divisble by 4, 3 or 2");
    }

    let condition = true;

    let second_number = if condition {5} else {6};
    println!("The value of second_number is {second_number}");

    let mut counter = 0;

    let result = loop {
        counter +=1;

        if counter == 10{

            break counter * 2; // allow to return las value of counter times 2s
        }
    };

    println!("The result is : {result}");

    let mut second_count = 0;

    'counting_up: loop{ // labeling the loop to be able to indetify it

        println!("second_count = {second_count}");
        let mut remaining = 10;

        loop{
            println!("Remaining = {remaining}");

            if remaining == 9{
                break;
            }

            if second_count == 2{

                break 'counting_up; // breaking the labeled loop thanks to its label
            }
            remaining -=1;
        }
        second_count += 1;
    }
    println!("End second_count = {second_count}");

    let mut second_number = 3;

    while second_number != 0{
        println!("{second_number}");
        second_number -= 1;

    }
    println!("LIFTOFF!!!");

    // Looping through a collection with for

    let a = [1, 2, 3, 4, 5];
    let mut index = 0;

    // Using while statement
    while index < 5{

        println!("The value is : {}", a[index]);
        index +=1;
    }

    // Using for statement
    
    for element in a{
        println!("The value of the element is : {element}");
    }

    // Using reverse method on ranges

    for number in (1..4).rev(){
        println!("Number : {number}");
    }
    println!("LIFTOFF!!!")

}
