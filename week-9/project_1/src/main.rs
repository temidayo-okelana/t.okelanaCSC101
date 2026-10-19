use std::io::Write;

fn main() {

   
    let lager = vec!["33 Export", "Desperados", "Goldberg","Gulder","Heineken","Star"];
    let stout = vec!["Legend", "Turbo King", "Williams","","",""];
    let non_alc = vec!["Maltina","Amstel Malta","Malta Gold","Fayrouz","",""];

    let mut file = std::fs::File::create("nigerian_breweries_catalogue.csv").expect("create failed");
    file.write_all("The Nigerian Brewereis Categories of Drinks\n".as_bytes()).expect("write failed");
    
    // as we are doing a table, we need csv elements
    // the wirte_all() function doesnt work for vectors of strings 
    // so we will loop through all vectors with a for loop and turn them into bytes
    
    let mut categories = String::new();

    let total_rows = lager.len(); // this is for the for loop, as we will loop as many times as the max column length

    for i in 0..total_rows {
        categories.push_str(lager[i]);
        categories.push_str(","); // this seperates the values by commas: 33 Export, Desparados,...

        categories.push_str(stout[i]);
        categories.push_str(",");

        categories.push_str(non_alc[i]);
        categories.push_str(",");

        categories.push_str("\n"); // this ends each row with a new line
    }

    file.write_all(categories.as_bytes()).expect("Unable to write data");
    
    println!("\nData written to file. ");

}