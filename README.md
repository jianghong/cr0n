crust
----
A scheduled process runner, like cron, but no cron; written in rust; readable scheduler definition in TOML.

````
crust
||
---- cron
 |
 ----rust
````

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

Usage
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
