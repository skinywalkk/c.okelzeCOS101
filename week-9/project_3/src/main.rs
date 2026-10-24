use std::fs::File;
use std::io::Write;

fn main() {
    let commissioners = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbonna",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];

    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    for i in 0..commissioners.len() {
        println!("{} | {} | {}", commissioners[i], ministries[i], zones[i]);
    }

    let mut file = File::create("efcc_ministers.txt").unwrap();
    for i in 0..commissioners.len() {
        writeln!(file, "{} | {} | {}", commissioners[i], ministries[i], zones[i]).unwrap();
    }
}
