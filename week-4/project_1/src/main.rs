use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();


    println!("Enter Value for a: ");
    io::stdin().read_line(&mut input1).expect("not a valid string");
    let vala:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter Value for b: ");
    io::stdin().read_line(&mut input2).expect("not a valid string");
    let valb:f32 = input2.trim().parse().expect("Not a valid number");
    
    println!("Enter Value for c: ");
    io::stdin().read_line(&mut input3).expect("not a valid string");
    let valc:f32 = input3.trim().parse().expect("Not a valid number");
    
    let d = valb * valb - 4.0 * vala * valc;

    if d > 0.0 {
        let r1 = (-valb + d.sqrt()) / (2.0 * vala);
        let r2 = (-valb - d.sqrt()) / (2.0 * vala);
        println!("The roots of the equation are {} and {}", r1, r2)
    }
     else if d == 0.0 {
        let r = -valb / (2.0 * vala);
        println!("root is {}", r);
     }
     else {
         println!("NO ROOTS");
     }
}
