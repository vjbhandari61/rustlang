use std::io;
use std::process;

fn take_input() -> (i32, i32) { // function to take inputs
    let mut input_1 = String::new();    // create two mutable strings to input values from users
    let mut input_2 = String::new();   

    println!("Enter number 1 below:");
    io::stdin().read_line(&mut input_1).expect("Failed to read line");  // input number 1
    println!("Enter number 2 below:");
    io::stdin().read_line(&mut input_2).expect("Failed to read line");  // input number 2

    let input_1a: i32 = input_1.trim().parse().expect("Not a valid number");    // convert number 1 from string to integer
    let input_2a: i32 = input_2.trim().parse().expect("Not a valid number");    // convert number 2 from string to integer
    return (input_1a, input_2a);    // return both values in a tuple for easy destructuring
}

fn main() {
    // start of the calculator program
    println!("Welcome to my CLI calculator!");  
    println!("1. Addition");
    println!("2. Subtraction");
    println!("3. Multiplication");
    println!("4. Division");

    let mut choice = String::new(); // mutable choice variable to choose the arithmetic action

    io::stdin() // input user's choice of action
        .read_line(&mut choice)
        .expect("Failed to read line");
    
    let choice_1: i32 = choice.trim().parse().expect("Not a valid number"); // convert the choice to integer as we'll be using integers for choosing the action.
    
    if choice_1 == 1 {      // check if the choice is 1 (Addition)
        let (input_1, input_2) = take_input();  // take inputs from the user using take_input function

        let sum: i32 = input_1 + input_2;   // add the two inputs together
        println!("Sum = {}", sum);  // print the result
        
    } else if choice_1 == 2 {   // check if the choice is 2 (Subtraction)
        let (input_1, input_2) = take_input();   // take user input 

        let diff: i32 = input_1 + input_2;
        /***** 
            // subtract the second number from the first one ( can result in negative as i32 also supports negative values)
            // we'd use the u32 if we are always sure that the resulting value will always be a positive value
        *****/
        println!("Diff = {}", diff);  // print the result
        
    } else if choice_1 == 3 {   // check if the choice is 3 (Multiplication)
        let (input_1, input_2) = take_input();  // take input from the user

        let product: i32 = input_1 * input_2;   // multiply the two values
        println!("Product = {}", product);  // print the product
        
    } else if choice_1 == 4 {   // check if the choice is 4 (Division)
        let (input_1, input_2) = take_input();  // take input from user

        let quotient: i32 = input_1 / input_2;  // divide the first number by the second and return the quotient
        println!("Quotient = {}", quotient);    // print the result
        
    } else {        // Any value other than 1, 2, 3 or 4, will be sent here
        println!("Invalid Choice"); // print that the choice is Invalid
    }

    // program execution stopped.
}
