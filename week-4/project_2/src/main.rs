use std::io;

fn main() {
    println!("You are experienced? (true/false):");
    let mut exp = String::new();
    io::stdin().read_line(&mut exp).expect("Not a valid string");
    let experience:bool = exp.trim().parse().expect("not a valid boolean");

    println!("Enter employee age");
    let mut ae = String::new();
    io::stdin().read_line(&mut ae).expect("Not a valid string");
    let age:i32 = ae.trim().parse().expect("Not a valid age");

    if experience{
        if age >= 40 {
            println!("Annual incentive: ₦1,560,000");
        } else if age >= 30 && age <= 39 {
            println!("Annual incentive: ₦1,480,000");
        } else if age < 28 {
            println!("Annual incentive: ₦1,300,000");
        } else {
            println!("Annual incentive: ₦1,300,000"); // fallback for 28–29
        }
    }
    else {
        println!("Annual incentive: ₦100,000");
    }

}
