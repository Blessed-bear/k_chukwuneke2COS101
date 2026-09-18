use std::io;

fn main() {

    println!("Enter the value of a:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid input");
    let a:f32 = input1.trim().parse().expect("Failed to read input");

    println!("Enter the value of b: ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid input");
    let b:f32 = input2.trim().parse().expect("Failed to read input");

    println!("Enter the value of c:");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Not the valid input");
    let c:f32 = input3.trim().parse().expect("Failed to read input");

    let discriminate = (b*b) -  (4.0*a*c);
    println!("The discriminate is: {}", discriminate);

    if discriminate > 0.0  {
        let root1 = (-b + discriminate.sqrt())  /  2.0 *  a;
        let root2 = (-b - discriminate.sqrt()) / 2.0 * a;
        println!("The two distinct numbers are {} and {}", root1, root2 );
    }else if discriminate == 0.0{
        let root1 = -b / 2.0  * a;
        println!("The dinstinct root is: {}",root1 );
    }else if discriminate < 0.0 {
        println!("The value is negative");
    }
}
