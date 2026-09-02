//Algo
/*START
INPUT principal
INPUT rate
INPUT time
COMPUTE amount after depreciation
PRINT amount after depreciation
STOP*/

fn main(){
	let p:f64 = 210_000.00;
	let r:f64 = 5.00;
	let t:f64 = 3.00;

	let a = p * (1.00 - (r/100.00)).powf(t);
	println!("The value after depreciation for 3 years = {:.2}", a );
}