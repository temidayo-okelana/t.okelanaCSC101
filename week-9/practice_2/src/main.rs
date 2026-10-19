use std::io::Read;

fn main() {
    // first create a txt file called 'welcome_message' by yourself and write something in it, so cargo can open and print
    let mut file = std::fs::File::open("welcome_message.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);
}