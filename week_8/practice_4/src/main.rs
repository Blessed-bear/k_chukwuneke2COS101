fn main() {

    // Name vector
    let name = vec!["Mary","Sam","Sally","Greg","Ade","Mark","June","Ife"];

    // Age vextor
    let age =  vec![16,17,19,22,20,21,18,23];

    println!("\n Age allocation:\n");

    // loop to iterate elements ion vectors
    for i in 0..age.len(){
        // iterating through i on vector
        print!("{} is {} yeears old\n",name[i],age[i] );
    }
}
