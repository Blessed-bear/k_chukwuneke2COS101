fn main() {
   
    // an array  of nnumbers 
    let numbers = [1,2,3,4,5];
    println!("Originsl array = {:?}", numbers);

    // create a slice of 2nd and 3rd element
    let slice1 = &numbers[1..3];
    println!("2nd and 3rd elements sliced = {:?}",slice1 );

    // omit the start index
    let slice2 = &numbers[..3];
    //  This means the slicee starts from index 0 annd goes up to index 3(exclusive)
    println!("index 0 to index 3 sliced =   {:?}",slice2 );

    // omit the start index
    let slice3 =   &numbers[2..];
    // This means tthe slice sttarts from index 2 and goes up  to index 5(exclusive)
    println!("index 2 to index 5 sliced  = {:?}",slice3 );

    // omit the start index and the eend index
    //   refeereence the whole array
    let slice4 = &numbers[..];
    //  This means the  slice starts from index 0 and goes too index 5(exclusive)
    println!("index 0 to index 5 sliced = {:?}", slice4);
}
