use std::io::Write;

fn main () {

    // writing the columns in different vectors
    let commissioner = vec!["Aigbogun Alamba Daudu","Murtala Afeez Bendu","Okorocha Calistus Ogbona","Adewale Jimoh Akanbi","Osazuwa Faith Etieye"];
    let ministry = vec!["Internal Affairs","Justice","Defense","Power & Steel","Petroleum"];
    let zone = vec!["South West","North East","South South","South West","South East"];

    // creating file
    let mut file = std::fs::File::create("EFCC_Convicted_Ministers.csv").expect("create failed");
    file.write_all("EFCC Convicted Ministers\n".as_bytes()).expect("write failed");
    file.write_all("Commissioner,Ministry,Zone\n".as_bytes()).expect("write failed");

    // looping all the rows into the format we need it to be
    let mut columns = String::new();
    let length = commissioner.len();

    for i in 0..length {
        columns.push_str(commissioner[i]);
        columns.push_str(",");

        columns.push_str(ministry[i]);
        columns.push_str(",");

        columns.push_str(zone[i]);
        columns.push_str(",");

        columns.push_str("\n");
    }

    // writing it all to the file
    file.write_all(columns.as_bytes()).expect("write failed");

    println!("\nData written to file");

}