use fluence::sdk::*;

fn init() {
    logger::WasmLogger::init_with_level(log::Level::Info).unwrap();
}

#[invocation_handler(init_fn = init, side_modules = (sqlite, redis))]
fn run(arg: String) -> Vec<u8> {
    let cmd: Vec<_> = arg.split(" ").collect();
    if cmd.len() < 2 {
        return "incorrect command".as_bytes().to_vec();
    }

    match cmd[0] {
        "redis" => redis::call(cmd[1].as_bytes()),
        "sqlite" => sqlite::call(cmd[1].as_bytes()),
        _ => "unknown_command".as_bytes().to_vec(),
    }
}