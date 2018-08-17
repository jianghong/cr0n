use std::process::Command;
use std::str;

pub fn run(program: &str, arg: Option<&str>) -> Result<String, &'static str> {
	let output = match arg {
		Some(a) => {
			match Command::new(program).arg(a).output() {
				Ok(o) => o,
				Err(_) => return Err("Error running program."),
			}
		},
		None => {
			match Command::new(program).output() {
				Ok(o) => o,
				Err(_) => return Err("Error running program."),
			}
		},
	};

	let output_str = match str::from_utf8(&output.stdout) {
		Ok(o) => o,
		Err(_) => return Err("Error converting to string."),
	};
	
	Ok(output_str.to_string())
}