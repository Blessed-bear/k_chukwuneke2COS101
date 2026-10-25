use std::io::Write;
fn main() {

    // Create and open the file for writing
    let mut  file = std::fs::File::create("EFCC_records.csv").expect("create failed");
    // write the csv column headers
    file.write_all("PAU SMIS\n".as_bytes()).expect("write failed");
    file.write_all("S/N,Name of Commissioner,Ministry,Geopolitical Zone\n".as_bytes()).expect("write failed");
    println!("\n Data written to file.");

    let commissioner: Vec<&str> =  vec!["Aigbogun Alama Daudu","Murtala Afeez Bendu","Okorocha Calistus Ogbona","Adewale Jimoh Akanbi","Osazuwa Faith Etieye"];
    let ministry: Vec<&str> = vec!["Internal Affairs","Justice","Defense","Power &  Steel","Petroleum"];
    let zone: Vec<&str> = vec!["South West","North East","South South","South West","South East"];

    for i in 0..commissioner.len() {
        let name = commissioner[i];
        let min = ministry[i];
        let z = zone[i];


        // Construct CSV row line and write to file
        let row = String::from(name) + "," + min + "," + z  + "\n";
        file.write_all(row.as_bytes()).expect("write failed");

        println!("{:<15} | {:<15} | {:<15}",name,min,z);
    }
   println!("\n Datasets successfully merged and saved to EFCC_records.csv!");

}
