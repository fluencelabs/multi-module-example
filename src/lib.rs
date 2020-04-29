use fluence::sdk::*;

const REDIS_OPTION_KEYWORD: &str = "redis";
const SQLITE_OPTION_KEYWORD: &str = "sqlite";

fn init() {
    logger::WasmLogger::init_with_level(log::Level::Info).unwrap();
}

#[invocation_handler(init_fn = init, side_modules = (redis, sqlite))]
fn run(arg: String) -> Vec<u8> {
    if arg.starts_with(REDIS_OPTION_KEYWORD) {
        log::info!("calling redis");
        redis::call(arg[REDIS_OPTION_KEYWORD.len() + 1..].as_bytes())
    } else if arg.starts_with(SQLITE_OPTION_KEYWORD) {
        log::info!("calling sqlite");
        redis::call(arg[SQLITE_OPTION_KEYWORD.len() + 1..].as_bytes())
    } else {
        Vec::from("unknown command")
    }
}
