# Multi module example
This is an example of multi modules feature of the Fluence Compute Engine (FCE). It simply passes commands to Redis or SQlite depends on the first parameter:
```
sqlite <sql> => returns result of execution sql request of SQlite
redis <cmd>  => the same for Redis 
``` 

# How to build

To build you need Rust with wasm32-unknown-unknown target installed, then just type:

```bash
cargo build --release --target wasm32-unknown-unknown
```

# How to run

To run this example you need FCE and [Redis](https://github.com/fluencelabs/redis/releases/download/0.8.0_w/redis.wasm) and [SQlite](https://github.com/fluencelabs/sqlite/releases/download/0.4.0_w/sqlite3.wasm) wasm binaries.

Then run fce and at first load sqlite and redis and only then this example:

```bush
$> Welcome to the Frank CLI:
$> >> add redis redis.wasm
$> module successfully registered in Frank
$> >> add sqlite sqlite3.wasm
$> module successfully registered in Frank
$> >> add mm mm.wasm
$> module successfully registered in Frank
``` 

Finally, you could execute requests inside FCE CLI:

```bush
$> >> execute redis SET A 1
result: +OK
$> >> execute redis GET A
result: $1
1
$> >> execute sqlite CREATE VIRTUAL TABLE users USING FTS5(body)
result: OK
$> >> execute sqlite INSERT INTO users(body) VALUES('AB'), ('BC'), ('CD'), ('DE')
result: OK
$> >> execute sqlite SELECT * FROM users WHERE users MATCH 'A* OR B*'
result: AB|BC
```
