fn main() {
	let p:f64 = 520_000_000.0;
	let n:f64 = 5.0;
	let r:f64 = 10.0;

	let a = p * (1 + (r/100)).pow(n);
	println!("The total amount is {}", a );
	let ci = a - p;
	println!("The compound interest is {}",ci );

}