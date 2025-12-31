use std::io; // import for inputs outputs
use rand::Rng;  // import for random number generation
use std::cmp::Ordering; // import to compare values of guessed number and random number

fn main() {
    // start of program
    println!("Guess the number!");
    println!("Guess any valid number between 1 to 10 and we'll see how strong your intuition is ^3^");

    // generate a random number between 1 and 10. 
    let rand_number = rand::thread_rng().gen_range(1..=10);

    // start loop from 0 to 2. It includes the starting number and excludes the end number. Thus it'll work for 0, 1, 2 = 3 times.
    for i in 0..3 {

        /*
            @dev_note: We are initializing the 'choice' inside the loop so that it reinitilizes the variable for every iteration.
            Faced an issue earlier when I was initializing the variable outside the loop: 
            'read_line()' function keeps appending the new values to the same memory. if choice 1 = 10 and choice 2 = 4, then the value in the memory of variale &choice = 10\n2\n. The whitespaces in the beginning and end would be removed with 'trim()' but the value would still not be an integer and the program would error. This is a feature of Rust to not clear memory implicitly.
        */

        let mut choice = String::new(); // create variable to store the user input
        io::stdin().read_line(&mut choice).unwrap();   // used unwrap here. It will quickly return the inner value of the variable but also panics quickly if the value is None or Err Option/Result. In production, we'll use safer explicit error handling using match, as done in the following lines.

        let choice: u32 = match choice.trim().parse() { // used match to trim the whitespaces and parse the string value into an unsigned integer (always positive value)
            Ok(num) => num, // handles the Ok Option of the choice variable. returns the integer value if present.
            Err(_) => { 
                println!("Please enter a valid number");    // breaks the program if the value is not an integer.
                return;
            }
        };

        match choice.cmp(&rand_number) {    // compare values of the choice and the random number generated. Will result in a Result Type.
            Ordering::Less => {     // check if the choice is lesser than rand_number
                println!("lesser than the number"); // prints lesser
                if i >= 2 { // if i greater or equal to 2 then quit the program
                    println!("You lost! Better luck next time!")  
                } else {    // else print the number of chances left
                    println!("There are {} chances left", 3 - (i+1));
                }
            },
            Ordering::Greater => {  // check if the choice is greater than rand_number
                println!("greater than the number");    // prints greater
                if i >= 2 {  // if i greater or equal to 2 then quit the program
                    println!("You lost! Better luck next time!")
                } else {    // else print the number of chances left
                    println!("There are {} chances left", 3 - (i+1));
                }    
            },
            Ordering::Equal => {    // check if the choice is equal to the rand_number
                println!("you guessed correctly! YOU WON!");    // print that the user has won
                break;  // end the loop
            },
        }
    }

    // End of the program
}
