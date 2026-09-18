use std::io;

fn main() {

    println!("Enter your name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Not a valid input");
    println!("My name is: {}", name );
    

    println!("Enter your age:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a  valid input");
    let age:i32 = input2.trim().parse().expect("Failed to read this users age");

    println!("Are you experienced? (yes/no)");
    let mut input3 = String ::new();
    io::stdin().read_line(&mut input3).expect("Not a valid input");
    let exp:bool =  input3.trim().to_lowercase()  ==  "yes";
    if age >= 40 && exp{
        println!("You  fall in the annual category of 1,560,000 ");
    }else if age >= 30 && age < 40 && exp{
        println!("You fall in the annual category of 1,480,000");
    } else if age < 30 && exp {
        println!("You fall into the annual category of 1,300,000");
    } else {
        println!("You fall into the annual category of 100,000");
    }
}
