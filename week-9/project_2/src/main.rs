use std::io::Write;

fn main() {
    
    // creating table contents by row
    let rows = vec![
        ["Student Name","Matric Number","Department","Level"],
        ["Oluchi Mordi","ACC10211111","Accounting","300"],
        ["Adams Aliyu", "ECO10110101","Economics","100"],
        ["Shania Bolade", "CSC10328828","Computer","200"],
        ["Adekunle Gold","EEE11020202","Electrical","200"],
        ["Bianca Edemoh","MEE10202001","Mechanical","100"],
    ];

    // creating the csv file
    let mut file = std::fs::File::create("PAU_SIMS.csv").expect("create failed");
    file.write_all("PAU SIMS\n".as_bytes()).expect("write failed");

    let mut line = String::new();

    for row in rows {
        line.push_str(row[0]);
        line.push_str(","); // this seperates the values by commas, as we are saving it in the format of csv

        line.push_str(row[1]);
        line.push_str(",");

        line.push_str(row[2]);
        line.push_str(",");

        line.push_str("\n"); // this ends each row with a new line
    }

    file.write_all(line.as_bytes()).expect("Unable to write data");
    
    println!("\nData written to file. ");

}