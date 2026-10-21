use std::io::Write;
use std::fs::OpenOptions;
fn main() {
     let announce = "Week  9 - Rust File Input & Output\n";
    let dept = "Department of Computer Science";

    /*let mut  file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all("Welcome to Rust Programming\n".as_bytes()).expect("write failed");
    file.write_all(announce.as_bytes()).expect("write failed");
    file.write_all(dept.as_bytes()).expect("write failed");
    println!("\n Data written to file.");*/

    let mut file = OpenOptions::new().append(true).open("data.txt").expect("cannot open file");
    file.write_all("\n Hello Class".as_bytes()).expect("write failed");
    file.write_all("\n This is appendage  to the document.".as_bytes()).expect("write failed");
    println!("file append success");
}
