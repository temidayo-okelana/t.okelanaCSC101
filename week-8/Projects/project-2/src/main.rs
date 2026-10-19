// a program that collects the name and years of experience of candidates
// and chooses the highest one

use std::io;

fn main() {

    // creating a vector of tuples where we push inputted values (name,years)
    let mut interviewees: Vec<(String,u32)> = Vec::new();
    let mut max_years:u32 = 0;
    let mut highest_index:usize = 0;

    println!("Welcome to the Interview Program for EY Global Limited!");

    // this asks how many candidates should be interviewed
    let mut input1 = String::new();
    println!("How many candidates would you like to interview today?");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let cand_num:usize = input1.trim().parse().expect("Invalid input");
    
    println!("\nWelcome, candidates!\n");
    
    // this for loop collects the names and years of experience of the candidates
    // it also pushes them into the string and gets the person with the most years of experience
    for i in 0..cand_num {
        let mut input2 = String::new();
        println!("Enter your name:");
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let int_name:String = input2.trim().parse().expect("Invalid input");

        let mut input3 = String::new();
        println!("Enter how many years of experience you have:");
        io::stdin().read_line(&mut input3).expect("Failed to read input");
        let int_years:u32 = input3.trim().parse().expect("Invalid input");

        println!("\nParticipant {}:\nName - {}\nYears of Experience - {}\n",i+1,int_name,int_years);

        if int_years > max_years {
            max_years = int_years;
            highest_index = i;
        }

        interviewees.push((int_name,int_years));  

    }

    // this part uses the index of the person witht the most years of experience from the for loop
    // and gets it from the vector while congratulating them
    let (best_name, best_years) = &interviewees[highest_index];
    println!("After consideration, we have decided to give the job to {} for having {} years of experience",best_name,best_years);
    println!("We are looking for candidates with many years of experience.");
    println!("We may still have more job opportunites, so stay alert, and thanks for applying!");

}