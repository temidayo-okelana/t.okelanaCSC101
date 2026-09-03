fn main() {
	// declaring variables
	let p:f64 = 210000.0;
	let r:f64 = 5.0;
	let n:i32 = 3;

	// formula for depreciation
	let amt = p*(1.0-(r/100.0)).powi(n);

	// if we run the code normally and use {:.2}, the output will still be in long decimal places becasue rust rounds down.
	// we use this to round it up to 180048.75
	let a = (amt*100.0).round()/100.0;

	println!("The cost of the TV is {:.2}", a);
}