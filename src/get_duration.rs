use std::io;


pub fn get_duration() -> u64 {
	println!("please input  your minutes of duration");
	let mut input  = String::new();
	io::stdin().read_line(&mut input).unwrap();
	let number = input.trim().parse::<u64>().unwrap();
	return number;
}