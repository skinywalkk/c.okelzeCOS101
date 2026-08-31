fn main(){
	let p:f64 = 1000.0;
	let r:f64 = 1.0;
	let t:f64 = 2.0;

	let si = (p * r * t)/100;
	println!("simple intrest is {}",si );
	let a = p + si; 
    println!("Amount is {}", a);
}