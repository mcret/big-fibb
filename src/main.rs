mod fibonacci;
mod args;

use std::time::Instant;
use args::FibonacciArgs;
use clap::Parser;
use crate::fibonacci::fibonacci;

fn main() {
	let args = FibonacciArgs::parse();

	let mut start = Instant::now();
	let mut end = Instant::now();

	if args.time {
		start = Instant::now();
	}

	let result = fibonacci(args.n);

	if args.time {
		end = Instant::now();
	}
	if args.print_result {
		println!("{}", &result);
	}
	if args.print_digit_count {
		let i = result.to_str_radix(10).chars().count();
		println!("Digit count:\t{}", i);
	}
	if args.print_bytes {
		println!("Byte count:\t{}", result.to_bytes_be().len())
	}

	if args.time {
		println!("{:?}", end.duration_since(start))
	}
}
