fn main() {
    
    // mutable array
    let mut colors = ["red", "green", "yellow", "white"];

    println!("\nOriginal array =  {:?}", colors);

    // mutable slice
    let slided_colors = &mut colors[1..3];

    println!("First slice = {:?}",slided_colors );

    // change the value of the original slice at thee first slice
    slided_colors[1] = "purple";

    println!("Changed slice = {:?}",slided_colors );
}
