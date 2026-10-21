
use std::io::Write;


fn main() {
    // Create and open the file for writing
    let mut  file = std::fs::File::create("Nigeria Berwery Limited.csv").expect("create failed");
    // write the csv column headers
    file.write_all("Lager,stout,Non-alcoholic\n".as_bytes()).expect("write failed");
    println!("\n Data written to file.");

    // Vectors storing the categories of drinks
    let lager: Vec<&str> =  vec!["33 Export","Desperados","Goldberg","Gulder","Heineken","Star"];
    let stout: Vec<&str> = vec!["Legend","Turbo King","Williams"];
    let non_alcoholic: Vec<&str> = vec!["Maltina","Amstel Malta","Malta Gold","Fayrouz"];

    // maximum of vectors
    let max_rows = lager.len().max(stout.len()).max(non_alcoholic.len());

    // loop row by row and write drink items separated 
    for i in 0..max_rows {
        if i < lager.len() {
            lager[i]
        }
        else {""};
        if i < stout.len() {
            stout[i]
        }
        else{""};
        if i < non_alcoholic.len() {
            non_alcoholic[i]
        }
        else {""};
    }

}
