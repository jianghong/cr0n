use std::process::Command;
use std::str;

pub fn run(program: &str, arg: Option<&str>) -> (Result<(), &'static str>) {
	let child = match arg {
		Some(a) => {
			println!("Running program: {}", program);
			println!("With args: {}", a);		
			let split: Vec<&str> = a.split(" ").collect();	
			match Command::new(program).arg(split[0].trim_matches('\'')).arg(split[1].trim_matches('\'')).spawn() {
				Ok(o) => o,
				Err(_) => return Err("Error running program."),
			}
		},
		None => {
			match Command::new(program).spawn() {
				Ok(o) => o,
				Err(_) => return Err("Error running program."),
			}
		},
	};


	let output = child
	    .wait_with_output()
	    .expect("failed to wait on child");

	assert!(output.status.success());

	Ok(())
}