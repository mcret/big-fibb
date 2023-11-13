use clap::Parser;
use num_bigint::BigUint;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct FibonacciArgs{
	/// The _n_ for the _n_th Fibonacci number
	pub n: BigUint,

	/// Causes the result to be prited
	#[arg(short = 'p')]
	pub print_result: bool,

	/// Causes the length of the result to be printed
	#[arg(short = 'c')]
	pub print_digit_count: bool,

	/// Causes the length of the result in bytes to be printed
	#[arg[short = 'b']]
	pub print_bytes: bool,

	/// Times the calculation and prints the duration
	#[arg(short = 't')]
	pub time: bool,
}
