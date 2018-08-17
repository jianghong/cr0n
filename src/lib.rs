pub mod runner;

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn it_runs_and_returns_stdout() {
    	let filepath = "test/bins/hello";
    	let result = runner::run(filepath);
    	assert!(result.is_ok());
        assert_eq!("Hello world!\n", result.unwrap());
    }

    #[test]
    fn it_handles_invalid_filepath() {
    	let filepath ="404";
    	let result = runner::run(filepath);
    	assert!(result.is_err());
    	assert_eq!("Error running program.", result.unwrap_err());
    }
}
