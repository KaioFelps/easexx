use std::io;

use build_options::BuildOptions;

mod build_options;
mod commands;
mod common;

fn main() -> io::Result<()> {
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
