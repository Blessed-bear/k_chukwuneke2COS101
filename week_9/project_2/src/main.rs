use std::io::Write;

fn main() {

    // Create and open the file for writing
    let mut  file = std::fs::File::create("pau_smis.csv").expect("create failed");
    // write the csv column headers
    file.write_all("PAU SMIS\n".as_bytes()).expect("write failed");
    file.write_all("Student name,Matric number,Department,Level\n".as_bytes()).expect("write failed");
    println!("\n Data written to file.");

    let student_name: Vec<&str> =  vec!["Oluchi Mordi","Adams Aliyu","Shania Bolade","Adekunle Gold","Blanca Edemoh"];
    let matric_number: Vec<&str> = vec!["ACC10211111","ECO10110101","CSC10328828","EEE11020202","MEE10202001"];
    let department: Vec<&str> = vec!["Accounting","Economics","Computer","Eleectrical","Mechanical"];
    let level: Vec<i32> = vec![300,100,200,200,100];

    for i in 0..student_name.len() {
        let name = student_name[i];
        let matric = matric_number[i];
        let dept = department[i];
        let lvl = level[i].to_string();


        // Construct CSV row line and write to file
        let row = String::from(name) + "," + matric + "," + dept + "," + &lvl + "\n";
        file.write_all(row.as_bytes()).expect("write failed");

        println!("{:<15} | {:<15} | {:<15} | {:<5}", name,matric,dept,lvl);
    }
   println!("\n Data printed to  screen and saved to pau_smis.csv successfully!");
}
