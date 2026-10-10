// shape calculator: calculates the area or volume of a shape with inputted values

use std::io;
use std::f32::consts::PI; // this is so we can use pi in calculations as a f32 type

fn main()
{
    
    println!("Hello! Welcome to the shape calculator.");

    // asking what the user wants to calculate
    let mut input1 = String::new();
    println!("What are you planning to calculate: area, surface area or volume?");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let answer:String = input1.trim().to_lowercase();



    // checking the options given, if not, ends the code
    if answer == "area" || answer == "surface area" || answer == "volume" {
        println!("");
    }
    else {
        println!("Please try again and enter one of the above options.");
        return; // the return keyword skips all other parts of the code, so the user cna try and input again
    }


    // this large if statemnet gets what shape we will calculate the area, sufeace area or volume of
    // the nested ifs check if the user input is valid, we cannot put the validity if statemnts outside the loops
    // storing our if statemnt in a variable allows the values inputted to be used outside the if statement
    let shape:String = if answer == "area" {   
        let mut input2 = String::new();
        println!("These are the shapes we can calculate the area of currently.\nWhich would you want: trapezium, rhombus, parallelogram");
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let area:String = input2.trim().to_lowercase();

        // this checks if the user input is valid
        if area == "trapezium" || area == "rhombus" || area == "parallelogram" {
            area  // if this is what is selected, this will be the value of shape, hence no need for semicolons, as it comes at the end of the if statement. the same applies to surface_area and volume
        }
        else {
            println!("We do not have this shape yet. Please enter one of the available shapes");
            return;
        }
    }

    else if answer == "surface area" {
        let mut input3 = String::new();
        println!("These are the shapes we can calculate the surface area of currently.\nWhich would you want: cube, cone");
        io::stdin().read_line(&mut input3).expect("Failed to read input");
        let surface_area:String = input3.trim().to_lowercase();

        // this checks if the user input is valid
        if surface_area == "cube" || surface_area == "cone" {
            surface_area
        }
        else {
            println!("We do not have this shape yet. Please enter one of the available shapes");
            return;
        }
    }

    else {
        let mut input4 = String::new();
        println!("These are the shapes we can calculate the volume of currently.\nWhich would you want: cylinder, sphere");
        io::stdin().read_line(&mut input4).expect("Failed to read input");
        let volume:String = input4.trim().to_lowercase();

        // this checks if the user input is valid
        if volume == "cylinder" || volume == "sphere" {
            volume
        }
        else {
            println!("We do not have this shape yet. Please enter one of the available shapes");
            return;
        }
    };  // when storing an if statement in a variable, a semicolon is needed after it



    // this if statement checks the shape and values inputted by the user, and gives them to the coressponding shape function to calculate
    if shape == "trapezium" {
        
        let mut dimension1 = String::new();
        println!("Please enter the length of the height of your trapezium in centimetres:");
        io::stdin().read_line(&mut dimension1).expect("Failed to read input");
        let trap_height:f32 = dimension1.trim().parse().expect("Invalid input");

        let mut dimension2 = String::new();
        println!("Please enter the length of the first parallel side of your trapezium in centimetres:");
        io::stdin().read_line(&mut dimension2).expect("Failed to read input");
        let trap_base1:f32 = dimension2.trim().parse().expect("Invalid input");

        let mut dimension3 = String::new();
        println!("Please enter the length of your second parallel sidfe of your trapezium in centimetres");
        io::stdin().read_line(&mut dimension3).expect("Failed to read input");
        let trap_base2:f32 = dimension3.trim().parse().expect("Invalid input");

        trapezium(trap_height,trap_base1,trap_base2)

    }
    
    else if shape == "rhombus" {

        let mut dimension4 = String::new();
        println!("Please enter the length of the first diagonal of your rhombus in centimetres:");
        io::stdin().read_line(&mut dimension4).expect("Failed to read input");
        let rhom_diagonal1:f32 = dimension4.trim().parse().expect("Invalid input");

        let mut dimension5 = String::new();
        println!("Please enter the length of the second diagonal of your rhombus in centimetres:");
        io::stdin().read_line(&mut dimension5).expect("Failed to read input");
        let rhom_diagonal2:f32 = dimension5.trim().parse().expect("Invalid input");

        rhombus(rhom_diagonal1,rhom_diagonal2)

    }

    else if shape == "parallelogram" {

        let mut dimension6 = String::new();
        println!("Please enter the length of the base of your parallelogram in centimetres:");
        io::stdin().read_line(&mut dimension6).expect("Failed to read input");
        let para_base:f32 = dimension6.trim().parse().expect("Invalid input");

        let mut dimension7 = String::new();
        println!("Please enter the length of the height of your parallelogram in centimetres:");
        io::stdin().read_line(&mut dimension7).expect("Failed to read input");
        let para_height:f32 = dimension7.trim().parse().expect("Invalid input");

        parallelogram(para_base,para_height)

    }

    else if shape == "cube" {

        let mut dimension8 = String::new();
        println!("Please enter the length of one of the sides of your cube in centimetres:");
        io::stdin().read_line(&mut dimension8).expect("Failed to read input");
        let cube_side:f32 = dimension8.trim().parse().expect("Invalid input");

        cube(cube_side)
    }

    else if shape == "cylinder" {

        let mut dimension9 = String::new();
        println!("Please enter the length of the radius of your cylinder in centimetres:");
        io::stdin().read_line(&mut dimension9).expect("Failed to read input");
        let cyli_radius:f32 = dimension9.trim().parse().expect("Invalid input");

        let mut dimension10 = String::new();
        println!("Please enter the length of the height of your cylinder in centimetres:");
        io::stdin().read_line(&mut dimension10).expect("Failed to read input");
        let cyli_height:f32 = dimension10.trim().parse().expect("Invalid input");

        cylinder(cyli_radius,cyli_height)

    }

    else if shape == "cone" {

        let mut dimension11 = String::new();
        println!("Please enter the length of the radius of your cone in centimetres:");
        io::stdin().read_line(&mut dimension11).expect("Failed to read input");
        let cone_radius:f32 = dimension11.trim().parse().expect("Invalid input");

        let mut dimension12 = String::new();
        println!("Please enter the length of the slant height of your cone in centimetres:");
        io::stdin().read_line(&mut dimension12).expect("Failed to read input");
        let cone_s_height:f32 = dimension12.trim().parse().expect("Invalid input");

        cone(cone_radius,cone_s_height)

    }

    else {

        let mut dimension13 = String::new();
        println!("Please enter the length of the radius of your sphere in centimetres:");
        io::stdin().read_line(&mut dimension13).expect("Failed to read input");
        let sphr_radius = dimension13.trim().parse().expect("Invalid input");

        sphere(sphr_radius)

    }

    println!("Thank you for using the shape calculator! Please come back when you need help!");

}

fn trapezium(h:f32, b1:f32, b2:f32) {
    let a:f32 = ((b1 + b2) * h) / 2.0;
    println!("Your trapezium has an area of {} squared centimetres.",a);
}

fn rhombus(d1:f32, d2:f32) {
    let a:f32 = 0.5 * d1 * d2;
    println!("Your rhombus has an area of {} squared centimetres.",a);
}

fn parallelogram(b:f32, h:f32) {
    let a:f32 = b * h;
    println!("Your parallelogram has an area of {} squared centimetres.",a);
}

fn cube(s:f32) {
    let s_a = 6.0 * s * s;
    println!("Your cube has a surface area of {} squared centimetres.",s_a);
}

fn cylinder(r:f32, h:f32) {
    let v:f32 = PI * r * r * h;
    println!("Your cylinder has a volume of {:.2} cubed centimetres.",v);  // the {:.2} rounds the area to 2 d.p as pi will give a long decimal answer
}

fn cone(r:f32, l:f32) {
    let s_a:f32 = (PI * r) * (r + l);
    println!("Your cone has a total surface area of {:.2} squared centimetres.",s_a);
}

fn sphere(r:f32) {
    let v = (4.0/3.0) * PI * r.powf(3.0); // we are raising a float to a power, so we use powf() instead of pow()
    println!("Your sphere has a volume of {:.2} cubed centimeres:",v);
}