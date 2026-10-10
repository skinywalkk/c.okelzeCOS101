use std::io;
fn trap(){
    let mut height = String::new();
    let mut base1 = String::new();
    let mut base2 = String::new();

    println!("Trapezium height: ");
    io::stdin().read_line(&mut height).expect("failed to read input");
    let height:f32 = height.trim().parse().expect("Invalid input");

    println!("Trapezium base1: ");
    io::stdin().read_line(&mut base1).expect("failed to read input");
    let base1:f32 = base1.trim().parse().expect("Invalid input");

    println!("Trapezium base2: ");
    io::stdin().read_line(&mut base2).expect("failed to read input");
    let base2:f32 = base2.trim().parse().expect("Invalid input");

    let area = (height / 2.0) * (base1 + base2);
    println!("Trapezium Area = {}", area);
}

fn rhomb(){
    let mut diag1 = String::new();
    let mut diag2 = String::new();

    println!("First Diagonal: ");
    io::stdin().read_line(&mut diag1).expect("failed to read input");
    let diag1:f32 = diag1.trim().parse().expect("Invalid input");

    println!("Second Diagonal: ");
    io::stdin().read_line(&mut diag2).expect("failed to read input");
    let diag2:f32 = diag2.trim().parse().expect("Invalid input");

    let area = 0.5 * diag1 * diag2;
    println!("Rhombus area = {}",area);
}

fn parallgrm(){
    let mut base = String::new();
    let mut altitude = String::new();

    println!("Parallelogram base:");
    io::stdin().read_line(&mut base).expect("failed to read input");
    let base:f32 = base.trim().parse().expect("Invalid input");

    println!("Parallelogram altitude");
    io::stdin().read_line(&mut altitude).expect("failed to read input");
    let altitude:f32 = altitude.trim().parse().expect("Invalid input");

    let area = base * altitude;
    println!("Parallelogram Area: {}",area);
}

fn cube(){
    let mut side1 = String::new();
    let mut side2 = String::new();

    println!("First side:");
    io::stdin().read_line(&mut side1).expect("failed to read input");
    let side1:i32 = side1.trim().parse().expect("Invalid input");

    println!("Second side:");
    io::stdin().read_line(&mut side2).expect("failed to read input");
    let side2:i32 = side2.trim().parse().expect("Invalid input");

    let area = 6 * side1 * side2;
    println!("Surface Area: {}",area);
}

fn cylind(){
    let mut radius = String::new();
    let mut height = String::new();

    println!("cylinder radius:");
    io::stdin().read_line(&mut radius).expect("failed to read input");
    let radius:f32 = radius.trim().parse().expect("Invalid input");

    println!("Cylinder height");
    io::stdin().read_line(&mut height).expect("failed to read input");
    let height:f32 = height.trim().parse().expect("Invalid input");

    use std::f32::consts::PI;
    let volume  = PI * radius * height * radius;
    println!("Cylinder Volume: {}", volume);
}



fn main() {
    let mut input1 = String::new();
    let shape_array =["Trapezium","Rhombus","Parallelogram","Cube","Cylinder"];
    println!("Pick a shape for calculations: {:?}", shape_array);
    io::stdin().read_line(&mut input1).expect("failed to read input");
    let shape = input1.trim();

    match shape {
        "Trapezium" => trap(),
        "Rhombus" => rhomb(),
        "Parallelogram" => parallgrm(),
        "Cube" => cube(),
        "Cylinder" => cylind(),
        _ => println!("Invalid choice!"),
     }
}
