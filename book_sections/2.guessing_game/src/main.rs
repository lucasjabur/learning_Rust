use std::io; // the standard library 'std' with the 'io' library
use std::cmp::Ordering; // the standard library 'std' with the 'cmp' library that has the 'Ordering' trait
use rand::Rng; // the library crate 'rand' that has the 'Rng' trait

fn main() {
    println!("Guess the number!"); // print a string    println!("Name: ");


    let secret_number = rand::thread_rng().gen_range(1..=100); // creates a not mutable variable, called "secret_number" of type u:32 (unsigned 32-bit integer)
                                                                    // of value rand::thread_rng().gen_range(1..=100).
                                                                        // it calls the library crate "rand" that has the "thread_rng" function.
                                                                        // this function has a method called "gen_range".
                                                                        // this method receives a start number and a final number, such as: "gen_range(1..=100)"

    println!("The secret number is: {secret_number}"); 

    loop { // initialize a loop

        println!("Please input your guess!");
        let mut guess = String::new(); // creates a mutable variable called "guess" of type String of value "String::new()"
                                               // "String" is a strign type provided by the Standard Library
                                               // "String::new()" calls the "new()" function that returns, in this case, a new instance of type "String"

        // std::io::stdin()
        io::stdin() // the "stdin()" function returns a instance of "std::io::Stdin", which is a type that represents a handle to the standard input for your
                         // terminal
            .read_line(&mut guess) // calls the "read_line()" function to get an input from the user.
                                                            // "&mut guess" is being passed as a parameter so that the program knows where to store the user input
                                                            // "&" indicates that the argument is a reference
                                                            // "mut" in being used because references are also immutable by default
            .expect("Failed to read line!"); // 'read_line()' returns a 'Result' value. The '.expect()' command deals with the error given by the 'Err' value of 'Result'. It will print the message inside the brackets and then crash the program.
        // could've been written like this as well:
        // io::stdin().read_line(&mut guess).expect("Failed to read line!");
    
        // let guess: u32 = guess.trim().parse().expect("Please type a number!"); // '.expect()' makes the program crash when an error is detected.
        let guess: u32 = match guess.trim().parse() { // instead of utilizing '.expect()', you can use the 'match' command to manipulate how the program will handle the
            Ok(num) => num,                      // error. The 'parse()' command returns a 'Result' value, that can be 'Err' (error) or 'Ok' ('guess' = num).
            Err(_) => continue,                       // if the 'Result' value is 'Err', it means that the user entered with a non-numerical value, and then the program
        };                                            // gives another chance to the user to enter a number. It goes on 'till a number is entered ('continue' statement).
        // 'match' statement -> 
        // '.trim()' -> eliminates any whitespaces at the beggining and at the end of a 'String'. It is necessary to compare a 'String' to a number. ' 42' != '42'.
        // '.parse()' -> converts a string to another type, in this case to an 'u32' type (unsigned 32-bit integer).


        println!("You guessed: {guess}"); // print the guessed number given by the user.
    
        match guess.cmp(&secret_number) { // it utilize the function '.cmp()' passing as it's parameter a reference (&) of the 'secret_number'.
            Ordering::Less => println!("Too small!"), // if the 'guess' is 'less than' the 'secret_number', than it will show to the user the message "Too small!"
            Ordering::Greater => println!("Too big!"), // if the 'guess' is 'greater than' the 'secret_number', than it will show to the user the message "Too big!"
            Ordering::Equal => { // now, it the 'guess' is 'equals to' the 'secret_number', than it will show to the user the message "You win!"
                println!("You win!"); 
                break; // and then a 'break' statement is used to break the 'loop' and the program ends.
            }
        }
    }    
}
