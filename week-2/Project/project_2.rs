//Algo 
/*START
INPUT amount for items
STORE amount for items
INPUT quantity of items
STORE quantity of items
COMPUTE sum of amount
COMPUTE average of amount
PRINT sum
PRINT average
STOP*/

fn main(){
	let t:f64 = 450_000.00;
	let m:f64 = 1_500_000.00;
	let h:f64 = 750_000.00;
	let d:f64 = 2_850_000.00;
	let a:f64 = 250_000.00;
	let qty:f64 = 10.00;

	let sum = (2.0*t) + m + (3.0*h) + (3.0*d) + a; //The total amounts of the material has to be calculated so I multiplied price by quantity
	let avg = sum/qty;
	println!("Sum = {}", sum);
	println!("Average = {}", avg);

}

