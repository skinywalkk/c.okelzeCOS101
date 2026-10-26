struct Laptop {
    brand: String,
    qty: i32,
    price: i32,
}

impl Laptop {
    fn total(&self) -> i32 {
        self.price * 3
    }
}

fn main() {
    let hp = Laptop {
        brand: String::from("HP"),
        qty: 10,
        price: 650000,
    };

    let ibm = Laptop {
        brand: String::from("IBM"),
        qty: 6,
        price: 755000,
    };

    let toshiba = Laptop {
        brand: String::from("Toshiba"),
        qty: 10,
        price: 550000,
    };

    let dell = Laptop {
        brand: String::from("Dell"),
        qty: 4,
        price: 850000,
    };

    let hp_cost = hp.total();
    let ibm_cost = ibm.total();
    let toshiba_cost = toshiba.total();
    let dell_cost = dell.total();

    let total = hp_cost + ibm_cost + toshiba_cost + dell_cost;

    println!("Cost for 3 {} laptops = N{}", hp.brand, hp_cost);
    println!("Cost for 3 {} laptops = N{}", ibm.brand, ibm_cost);
    println!("Cost for 3 {} laptops = N{}", toshiba.brand, toshiba_cost);
    println!("Cost for 3 {} laptops = N{}", dell.brand, dell_cost);
    println!("Total Cost = N{}", total);
}