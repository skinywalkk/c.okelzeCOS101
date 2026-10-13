use std::io;


fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn main() {
    
    let levels = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];

   
    let office_admin = vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"];
    let academic = vec!["-", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];
    let lawyer = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];
    let teacher = vec!["Placement", "Classroom Teacher", "Senior Teacher", "Leading Teacher", "Deputy Principal", "Principal"];

    println!("=== Federal Government of Nigeria: APS Level Checker ===\n");
    println!("1. Office Admin");
    println!("2. Academic");
    println!("3. Lawyer");
    println!("4. Teacher");

    let choice: usize = match read_input("\nChoose a profession (1 - 4):").parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid choice. Please enter a number from 1 to 4.");
            return;
        }
    };

    // a slice pointing at whichever profession vector was chosen
    let roles: &[&str] = match choice {
        1 => &office_admin,
        2 => &academic,
        3 => &lawyer,
        4 => &teacher,
        _ => {
            println!("Invalid choice. Please enter a number from 1 to 4.");
            return;
        }
    };

    println!("\nAvailable positions:");
    for i in 0..roles.len() {
        if roles[i] != "-" {
            println!("  - {}", roles[i]);
        }
    }

    let title = read_input("\nEnter your position:");

    // search the roles vector for the position (ignoring upper/lower case)
    let mut found = false;
    for i in 0..roles.len() {
        if roles[i] != "-" && roles[i].to_lowercase() == title.to_lowercase() {
            println!("\n{} holds position {}", roles[i], levels[i]);
            found = true;
            break;
        }
    }

    if !found {
        println!("\n\"{}\" is not a valid position for that profession.", title);
    }
}