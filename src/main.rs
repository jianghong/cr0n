use std::process::Command;
use std::str;

fn main() {
	let output = Command::new("test/bins/hello")
	                     .output()
	                     .expect("Failed to execute command");

	 let output_str = str::from_utf8(&output.stdout).expect("Failed to convert to string");
	 println!("{:?}", output_str);
}