fn main() {
    let fullname = "Chidubum John Umeh";
    let department = "Computer Science";
    let uni = "Pan-Atlantic University";

    // the .to_string() method turns any value into a string object
    let mut school = "School of Sciecne".to_string();
    // the .push_str() method adds a string slice to the end of an existing string
    school.push_str(" and Technology");

    println!("My name is: {}",fullname);
    println!("The length of my fullname is: {}",fullname.len());
    println!("I am a student of {} Department",department);
    println!("{}",school);
    println!("{}",uni);
}
