extern crate toml;
extern crate cr0n;

use std::fs;
use std::time::{Duration, SystemTime};
use std::thread::sleep;
use toml::Value;
use cr0n::runner;

fn main() {
	let schedule_contents = fs::read_to_string("test/schedule.toml").expect("Fail");
	let value = schedule_contents.parse::<Value>().unwrap();

	let ten_sec_programs = value["every_ten_seconds"].as_array().unwrap();
	let minute_programs = value["every_minute"].as_array().unwrap();

	let mut ten_seconds_clock = SystemTime::now();
	let mut minute_clock = SystemTime::now();

	let mut now;

	loop {
		now = SystemTime::now();
		if ten_seconds_checker(&ten_sec_programs, now, ten_seconds_clock) {
			ten_seconds_clock = now;
		}

		if minute_checker(&minute_programs, now, minute_clock) {
			minute_clock = now;
		}		

		sleep(Duration::new(1, 0));
	}
}

fn ten_seconds_checker(programs: &Vec<Value>, now: SystemTime, last: SystemTime) -> bool {
	if now.duration_since(last).expect("Time went wrong").as_secs() >= 10 {
		for prog in programs {
			let split: Vec<&str> = prog.as_str().unwrap().split(" ").collect();
			match runner::run(split[0], Some(split[1])) {
				Ok(o) => ( println!("Runner output: {}", o)),
				Err(_) => (),
			}
		}
		return true
	}
	false
}

fn minute_checker(programs: &Vec<Value>, now: SystemTime, last: SystemTime) -> bool {
	if now.duration_since(last).expect("Time went wrong").as_secs() >= 60 {
		for prog in programs {
			let split: Vec<&str> = prog.as_str().unwrap().split(" ").collect();
			match runner::run(split[0], Some(split[1])) {
				Ok(o) => ( println!("Runner output: {}", o)),
				Err(_) => (),
			}
		}
		return true
	}
	false
}