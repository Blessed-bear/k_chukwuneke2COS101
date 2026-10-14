fn main() {

    //Using Vec::new()
    let v: Vec<i64> = Vec::new();

    // printing the size of vector
    println!("\n The length of Vec::new is {}",v.len());

    // Using macro
    let v = vec!["Grace","Effiong","Basil","Kareem","Susan"];

    // Printing the size of vector
    println!("\n The length of Vec macro is : {}",v.len());
}
