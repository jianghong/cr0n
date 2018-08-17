Define a `.toml` file
````
every_ten_seconds = [
	"echo ten_seconds"
]

every_minute = [
	"echo minute"

````

Get output
````
➜  cr0n git:(master) ✗ cargo run
   Compiling cr0n v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 0.70s
     Running `target/debug/cr0n`
Runner output: ten_seconds

Runner output: ten_seconds

Runner output: ten_seconds

Runner output: ten_seconds

Runner output: ten_seconds

Runner output: ten_seconds

Runner output: minute

Runner output: ten_seconds

Runner output: ten_seconds

Runner output: ten_seconds

Runner output: ten_seconds

Runner output: ten_seconds

Runner output: ten_seconds

Runner output: minute
````
