use std::io;
fn checker(){
    let mut input = String::new();
    println!("Enter a character: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let ch:char =input.trim().parse().expect("Invalid input");

    if ch >= '0' && ch <= '9'{
        println!("character {} is a digit", ch);
    }
    else{
        println!("Character {} is NOT a digit", ch);
    }
}


fn main() {
    println!("Welcome, this program checks if the inputed character is a digit");
    checker();
}
