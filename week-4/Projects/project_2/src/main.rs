// rust program to print the annual incentive from given criteria

use std::io;

fn main()
{
    // the two criteria needed are the user's experience and how old they are
    // getting the user's experience
    let mut input1 = String::new();
    println!("Please type if you are 'experienced' or 'not experienced'. ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let experience = input1.trim().to_lowercase(); // the .to_lowercase() function changes the case of the answer to be lowercase, allowing for a capital answer


    // getting the user's age
    let mut input2 = String::new();
    println!("What is your age? ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age:u32 = input2.trim().parse().expect("Not a valid number");

    // running an if loop to see if the user is experienced or not
    // if they are experienced, a nested loop asks for their age
    // if they input neither of the two options, it tells them to
    if experience  == "experienced" {
        if age >= 40 {
            println!("Your annual incentive is ₦1,560,000");
        } else if 30 >= age & age && age & age >= 39 {
            println!("Your annual incentive is ₦1,480,000");
        } else {
            println!("Your annual incentive is ₦1,300,000");
        }
    } else if experience == "not experienced" {
        println!("Your annual incentive is ₦100,000");
    } else {
        println!("You have inputed the wrong information. Please type'experienced' or 'not experienced' and try again. ");
    }
}