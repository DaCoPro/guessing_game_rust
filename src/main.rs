use std::io;
//call the std library, specifically io tools
//above the main function

fn main() {
    println!("Guess the number!");
    
    println!("Please input your guess.");
    
    let mut guess = String::new();
    //mut is necessary to make a string mutable, 
    //::new() is a associated function of String
    
    let im_var = 5;
    //example of a standard variable
    
    io::stdin()
    //could also call in-line with std::io::Stdin
        .read_line(&mut guess)
        //directs stdin() to the 
        .expect("Failed to real line");

    println!("You guessed: {}", guess);
}
