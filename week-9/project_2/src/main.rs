use std::fs::File;
use std::io::Write;

fn main() {
    let students = vec![
        ("Oluchi Mordi", "ACC10211111", "Accounting", "300"),
        ("Adams Aliyu", "ECO10110101", "Economics", "100"),
        ("Shania Bolade", "CSC10328828", "Computer", "200"),
        ("Adekunle Gold", "EEE11020202", "Electrical", "200"),
        ("Blanca Edemoh", "MEE10202001", "Mechanical", "100"),
    ];
    println!("{:<15} {:<15} {:<15} {:<5}", "Student Name", "Matric Number", "Department", "Level");
    println!("{}", "-".repeat(60));
    for student in &students {
        println!("{:<15} {:<15} {:<15} {:<5}", student.0, student.1, student.2, student.3);
    }
    
    let mut file = File::create("students.txt").unwrap();
    writeln!(file, "{:<15} {:<15} {:<15} {:<5}", "Student Name", "Matric Number", "Department", "Level").unwrap();
    writeln!(file, "{}", "-".repeat(60)).unwrap();
    for student in &students {
        writeln!(file, "{:<15} {:<15} {:<15} {:<5}", student.0, student.1, student.2, student.3).unwrap();
    }

    println!("File 'students.txt' created successfully!");
}
