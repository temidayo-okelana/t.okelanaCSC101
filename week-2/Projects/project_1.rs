fn main() {
	// declaring variables
	let p:f64 = 520000000.0;
	let r:f64 = 10.0;
	let n:i32 = 5; // variables to the power should be in i32 and have no decimals


	// compound interest
	let a = p*(1.0+(r/100.0)).powi(n);
	println!("Amount is {:.2}", a); // the :.2 is so the calculations do not have weird decimal places
	let ci = a - p;
	println!("Compound Interest is {:.2}", ci);
}