use std::fs::File;
use std::io::Write;

fn main() {
    let lager = vec!["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"];
    let stout = vec!["Legend", "Turbo King", "Williams"];
    let non_alcoholic = vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"];

    let mut file = File::create("breweries.txt").unwrap();

    writeln!(file, "Lager\t\tStout\t\tNon-Alcoholic").unwrap();
    writeln!(file, "---------------------------------------------").unwrap();

    let max_len = if lager.len() > stout.len() && lager.len() > non_alcoholic.len() {
        lager.len()
    } else if stout.len() > non_alcoholic.len() {
        stout.len()
    } else {
        non_alcoholic.len()
    };

    for i in 0..max_len {
        let l = if i < lager.len() { lager[i] } else { "" };
        let s = if i < stout.len() { stout[i] } else { "" };
        let n = if i < non_alcoholic.len() { non_alcoholic[i] } else { "" };

        writeln!(file, "{}\t\t{}\t\t{}", l, s, n).unwrap();
    }
}
