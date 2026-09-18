// rust program to give the roots of a quadratic equation

use std::io;

fn main()
{
    // a quadratic equation is in the form ax^2 + bx + c = 0
    // getting a from the user
    let mut input1 = String::new();
    println!("Enter a value for a: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    // getting b from the user
    let mut input2 = String::new();
    println!("Enter a value for b: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid numnber");

    // getting c from the user
    let mut input3 = String::new();
    println!("Enter a value for c: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

    // setting up the determinant
    let d:f32 = b*b - 4.0*a*c;

    // calculating the quadratic roots
    let x_1 = (-b + d.sqrt()) / (2.0*a);
    let x_2 = (-b - d.sqrt()) / (2.0*a);

    // if statement for the conditions of the determinant
    if d < 0.0 {
        println!("There are no real roots to this equation.");
    } else if d == 0.0 {
        println!("There is one root, {}", x_1);
    } else {
        println!{"The roots to this equation are {} and {}", x_1, x_2};

    }
}