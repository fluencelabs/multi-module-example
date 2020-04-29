# Multi module example
This is an example of multi modules feature of the Fluence Compute Engine (FCE). It simply passes commands to Redis or SQlite depends on the first parameter:
```
sqlite <sql> => returns result of execution sql request by SQlite
redis <cmd>  => the same for Redis 
``` 

# How to build

To build you need Rust with wasm32-unknown-unknown target installed, then just type:

```bash
cargo build --release --target wasm32-unknown-unknown
```

# How to run

To run this example you need FCE, [Redis](https://github.com/fluencelabs/redis/releases/download/0.8.0_w/redis.wasm), [SQlite](https://github.com/fluencelabs/sqlite/releases/download/0.4.0_w/sqlite3.wasm) wasm binaries.

Then run fce and at first load sqlite and redis and only then this example:

```bush
Welcome to the FCE CLI:
>> add redis redis.wasm
module successfully registered in FCE
>> add sqlite sqlite3.wasm
module successfully registered in FCE
>> add mm mm.wasm
module successfully registered in FCE
``` 

Finally, you could execute requests inside FCE CLI:

```bush
>> execute mm redis SET A 1
result: +OK
>> execute mm redis GET A
result: $1
1
>> execute mm sqlite CREATE VIRTUAL TABLE users USING FTS5(body)
result: OK
>> execute mm sqlite INSERT INTO users(body) VALUES('AB'), ('BC'), ('CD'), ('DE')
result: OK
>> execute mm sqlite SELECT * FROM users WHERE users MATCH 'A* OR B*'
result: AB|BC
```
