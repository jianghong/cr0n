crust
----

````
crust
||
---- cron
 |
 ----rust
````

A scheduled process runner, like cron; written in rust; readable schedule defined in TOML.


Define a `.toml` file
----
````
every_ten_seconds = [
    "echo ten_seconds"
]

every_minute = [
    "echo minute"
]
````

Get output
----
````
➜  crust git:(master) ✗ cargo run
   Compiling crust v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 0.70s
     Running `target/debug/crust`
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
