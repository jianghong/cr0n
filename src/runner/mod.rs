use std::process::Command;
use std::str;

pub fn run(program: &str) -> Result<String, &'static str> {
	let output = match Command::new(program).output() {
		Ok(o) => o,
		Err(_) => return Err("Error running program."),
	};

	let output_str = match str::from_utf8(&output.stdout) {
		Ok(o) => o,
		Err(_) => return Err("Error converting to string."),
	};
	
	Ok(output_str.to_string())
}