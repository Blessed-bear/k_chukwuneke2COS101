use std::io;

fn main() {
    println!("\n Student Information Management System!");

    // input name
    println!("\n Please Enter your name.");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");
    println!("Your name is:  {}",name );

    // input age 
    println!("\nEnter your age");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Failed to read input");
    println!("I am {} years",age );
}
