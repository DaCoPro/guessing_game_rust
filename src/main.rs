use std::io;
//call the std library, specifically io tools
//above the main function
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    //these commands and associated functions can be located in crate docs. run cargo doc --open to see docs for depencies
    
    println!("The secret number is: {}", secret_number);
  
    loop {
        println!("Please Input your guess.");
        let mut guess = String::new();
        //mut is necessary to make a string mutable, 
        //::new() is a associated function of String
        
        // let im_var = ;
        //example of a standard variable
        //build will warn you about un-used variables
        
        io::stdin()
        //could also call in-line with std::io::Stdin
            .read_line(&mut guess)
            //directs stdin() to the variable we want it to utilize
            .expect("Failed to real line");
            //result types are enums
            //Result has variants Ok or Err
            //Ok contains successfully generated value
            //Err contains info about why operation failed
            //if the result is Err, the program will crash and display our message
            //without expect, you'll get a warning - YOU SHOULD HANDLE ALL POTENTIAL ERRRORS

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        //guess is shadowed from original. We can use the same variable name. 
        //parse parses a string type into some kind of number
        //match expression is basically a switch statement. Match statements are a way to evolve from .expect statements and actually handle the errors
        //underscore is a catch all value

        println!("You guessed: {}", guess);
        //the curlies indicate insertion points, arguments after string ends are their ordered contents
        //example: 
        // fn main() {
        // let x = 5;
        // let y = 10;

        // println!("x = {} and y = {}", x, y);
        // }
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    println!("You broke out of the looP!")
    //this code will run after the loop's been passed, without a break statement in the loop it would be 'unreachable expression'
}
