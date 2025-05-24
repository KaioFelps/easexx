use build_options::BuildOptions;
use std::io;

mod build_options;
mod commands;
mod common;

pub const SOURCE_DIR: &str = "src";
pub const TESTS_DIR: &str = "tests";
pub const DEFAULT_CONFIG_FILE: &str = "build.json";

fn main() -> io::Result<()> {
    let _ = dotenvy::dotenv();
    env_logger::builder()
        .filter_level(log::LevelFilter::Warn)
        .parse_env("RUST_ENV")
        .init();

    let args = std::env::args().collect::<Vec<_>>();

    let command = match args.get(1) {
        Some(command) => command,
        None => {
            return commands::man::exec();
        }
    };

    let options = BuildOptions::read_from_config_file()?;

    match command.as_str() {
        "build" => commands::build::exec(&options),
        "test" => commands::test::exec(&options),
        _ => {
            println!("Comando {command} não encontrado.");
            commands::man::exec()
        }
    }
}
