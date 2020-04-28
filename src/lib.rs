use fluence::sdk::*;

fn init() {
    logger::WasmLogger::init_with_level(log::Level::Info).unwrap();
}

#[invocation_handler(init_fn = init, side_modules = (redis, sqlite))]
fn run(arg: String) -> Vec<u8> {
    let cmd: Vec<_> = arg.split(" ").collect();
    if cmd.len() < 2 {
        return "please specify module and argument".as_bytes().to_vec();
    }

    let arg = cmd[1..].join(" ");

    match cmd[0] {
        "redis" => {
            log::info!("calling redis");
            redis::call(arg.as_bytes())
        },
        "sqlite" => {
            log::info!("calling sqlite");
            sqlite::call(arg.as_bytes())
        },
        _ => "unknown_command".as_bytes().to_vec(),
    }
}
