use std::io;

fn main() {
    println!(" MENU - FOOD ITEMS ");
    println!(" P = pounded yam / edikaiiko soup  - 3,200");
    println!(" F = fried rice & chicken  - 3,000");
    println!(" A = amala & ewedu soup  - 2,500");
    println!(" E = eba & egusi soup  - 2,000");
    println!(" W = white rice & stew  - 2,500");

    println!("\nPlease enter the letter of the item you want to order:");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");
    let choice = choice.trim().to_uppercase();

    let (food, price) = match choice.as_str() {
        "P" => ("Pounded yam / Edikaiiko soup", 3200),
        "F" => ("Fried rice & chicken", 3000),
        "A" => ("Amala & ewedu soup", 2500),
        "E" => ("Eba & egusi soup", 2000),
        "W" => ("White rice & stew", 2500),
        _ => {
            println!("Invalid choice! Please restart and enter a valid letter.");
            return;
        }
    };

    println!("\nYou ordered: {}", food);
    println!("Price: ₦{}", price);
    println!("Thank you for ordering from us. Have a great day!");
}
