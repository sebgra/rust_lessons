use std::io; // Call I/O library to manage input from the standard library
use rand::Rng; // Call random generators from rand library
use std::cmp::Ordering; // Call comparing crates from std library


fn main(){

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // request a random number beteween 0 and 100 using a range syntax

    println!("The secret number is : {secret_number}");


    loop { // define infinite loop
        println!("Please input your guess.");

        let mut guess = String::new(); // Instanciate guess to the type String provided by the std library, but let it empty.

        io::stdin().read_line(&mut guess).expect("Failed to read line"); // Instanciate new mutable variable with an empty String object passed through reference
                                                                        // guess will contain the typed string from the user.


        let guess: u32 = match guess.trim().parse(){// Hide previous definition of guess and bind it to whitespaces and \n trimmed user input. Str will be converted to u32 to be comprable to secret _number

                Ok(num) => num, // As an enum readline has two states : Ok everything has gone well. Here num is used to evaluate the case and is return if OK.
                Err(_) => continue, // Err something went wrong, the loop has to be continued
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