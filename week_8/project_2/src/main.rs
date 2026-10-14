    use std::io;

fn main() {
    // Create a vector of tuples to store (name, years_of_experience)
    let mut candidates: Vec<(String, u32)> = Vec::new();

    println!("EY Nigeria – Developer Experience Checker");

    loop {
        // Get candidate's name
        println!("\nEnter candidate's name:");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read name");
        let name = name.trim().to_string();

        // Get candidate's years of programming experience
        println!("Enter years of programming experience:");
        let mut exp = String::new();
        io::stdin().read_line(&mut exp).expect("Failed to read experience");
        let exp: u32 = exp.trim().parse().expect("Invalid number");

        // Store candidate info inside the vector
        candidates.push((name, exp));

        // Ask if another candidate should be entered
        println!("Do you want to enter another candidate? (yes/no)");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read choice");
        if choice.trim().to_lowercase() == "no" {
            break;
        }
    }

    // Find candidate with highest experience
    let mut best_candidate = ("None".to_string(), 0);

    for (name, exp) in &candidates {
        if *exp > best_candidate.1 {
            best_candidate = (name.clone(), *exp);
        }
    }

    println!("\n Candidate with highest experience:");
    println!("Name: {}", best_candidate.0);
    println!("Years of Experience: {}", best_candidate.1);
}