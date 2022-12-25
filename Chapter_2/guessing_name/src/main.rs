use std::io; // Call I/O library to manage input from the standard library
use rand::Rng; // Call random generators from rand library
use std::cmp::Ordering; // Call comparing crates from std library


fn main(){

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // request a random number beteween 0 and 100 using a range syntax

    println!("The secret number is : {secret_number}");


    loop { // define infinite loop
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line"); // Instanciate new mutable variable with an empty String object passed through reference

        let guess: u32 = match guess.trim().parse(){// Hide previous definition of guess and bind it to whitespaces and \n trimmed user input. Str will be converted to u32 to be comprable to secret _number

                Ok(num) => num,
                Err(_) => continue,
            };

        println!("You guessed : {guess}");

        match guess.cmp(&secret_number){

            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {println!("You win!"); // Stop script if correct guess
                                break;},
        }

    }

}