use std::io;

fn main() {
    
    // creating vectors for each column on the table
    let public_serv = vec!["APS 1-2","APS 3-5","APS 5-8","EL1 8-10","EL2 10-13","SES"];
    let office_admn = vec!["Intern","Administrator","Senior Administrator","Office Manager","Director","CEO"];
    let academic = vec!["N/A","Research Assistant","PhD Candidate","Post-Doc Researcher","Senior Lecturer"];
    let lawyer = vec!["Paralegal","Junior Associate","Associate","Senior Associate 1-2","Senior Associate 3-4","Partner"];
    let teacher = vec!["Placement","Classroom Teacher","Senior Teacher","Learning Teacher","Deputy Principal","Principal"];

    println!("Welcome to the APS Level Checker!");

    let mut input1 = String::new();
    println!("Which of the following departments are you in?\nOffice Admin, Academic, Lawyer, Teacher");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let department = input1.trim().to_lowercase();  


    // this if statement checks the department given, and asks for the job they do in the department, exactly as it is written
    let job = if department == "office admin" {   
        println!("");
        for office_admn in &office_admn {
            println!("{} ",office_admn);
        }

        let mut input2 = String::new();
        println!("Please select on of these as your jobs. Type them as they appear.");
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        input2.trim().to_string()
    }

    else if department == "academic" {
        println!("");
        for academic in &academic {
            println!("{} ",academic);
        }

        let mut input3 = String::new();
        println!("Please select on of these as your jobs. Type them as they appear.");
        io::stdin().read_line(&mut input3).expect("Failed to read input");
        input3.trim().to_string()
    }

    else if department == "lawyer" {
        println!("");
        for lawyer in &lawyer {
            println!("{} ",lawyer);
        }

        let mut input4 = String::new();
        println!("Please select on of these as your jobs. Type them as they appear.");
        io::stdin().read_line(&mut input4).expect("Failed to read input");
        input4.trim().to_string()
    }

    else if department == "teacher" {
        println!("");
        for teacher in &teacher {
            println!("{} ",teacher);
        }

        let mut input5 = String::new();
        println!("Please select on of these as your jobs. Type them as they appear.");
        io::stdin().read_line(&mut input5).expect("Failed to read input");
        input5.trim().to_string()
    }

    else {
        println!("That is not one of the departments we can check. Please try one of the stated departments.");
        return;
    };

    let mut input6 = String::new();
    println!("\nHow many years have you been working in this field?");
    io::stdin().read_line(&mut input6).expect("Failed to read input");
    let experience = input6.trim().parse().expect("Invalid input");

    let position_exp = if experience >= 1 && 3 > experience {
        0
    } 
    else if experience >= 3 && 5 > experience {
        1
    }
    else if experience >= 5 && 8 > experience {
        2
    }
    else if experience >= 8 && 10 > experience {
        3
    }
    else if experience >= 10 && 13 > experience {
        4
    }
    else {
        5
    };

    let position_job:Option<usize> = if department== "office admin" {
        office_admn.iter().position(|&r| r.to_lowercase() == job.to_lowercase())
    }
    else if department == "academic" {
        academic.iter().position(|&r| r.to_lowercase() == job.to_lowercase())
    } 
    else if department == "lawyer" {
        lawyer.iter().position(|&r| r.to_lowercase() == job.to_lowercase())
    }
    else {
        teacher.iter().position(|&r| r.to_lowercase() == job.to_lowercase())
    };

    // the match statement checks the indexes of both the job and experience by using tuples, then matches them to the public_serv vector
    match (position_job,position_exp) {
       (Some(job_idx),exp_idx) => {
        let level_by_experience = public_serv[exp_idx];
        let level_by_job = public_serv[job_idx];

        println!("Based on your job, your APS level is {}.",level_by_job);
        println!("Based on your experience, your APS level is {}.",level_by_experience);

        if level_by_job == level_by_experience {
            println!("Your APS Levels match in both fields of job and expertise.");
        }
        else {
            println!("Your APS Levels do not match in the fields of job and expertise. Try checking what you inputted.");
        }
       }

       (None, _) => {
        println!("This is not one of the jobs available. Please select out of what is displayed.");
        return;
       }
    }

}
