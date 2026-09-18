//Rust program to calculate the area of a triangle given three sides

use std::io;

fn main() {
    // declaring the variables for three sides of the triangle
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    // first side
    println!("Enter first edge of triangle: ");
    io::stdin().read_line(&mut input1).expect("Not a valid number");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

     // second side
    println!("Enter second edge of triangle: ");
    io::stdin().read_line(&mut input2).expect("Not a valid number");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

     // third side
    println!("Enter third edge of triangle: ");
    io::stdin().read_line(&mut input3).expect("Not a valid number");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

    let s:f32 = (a*b*c)/2.0;
    let mut area:f32 = s*(s-a)*(s-b)*(s-c);
    area = area.sqrt();

    println!("Area of a triangle: {}", area);
}