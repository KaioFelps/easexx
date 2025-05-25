use build_options::BuildOptions;
use commands::EasexxCommand;
use utils::{get_build_options, get_command_from_args, GetBuildOptionsArgs};

mod build_options;
mod commands;
mod common;
mod utils;

#[cfg(test)]
mod test_utils;

pub const SOURCE_DIR: &str = "src";
pub const TESTS_DIR: &str = "tests";
pub const DEFAULT_CONFIG_FILE: &str = "build.json";

fn main() {
    let _ = dotenvy::dotenv();
    env_logger::builder()
        .filter_level(log::LevelFilter::Warn)
        .format_file(false)
        .format_timestamp(None)
        .parse_env("RUST_ENV")
        .init();

    let args = std::env::args().collect::<Vec<_>>();

    let command = get_command_from_args();

    let options = match get_build_options(GetBuildOptionsArgs {
        args: &args,
        default_config_file_path: DEFAULT_CONFIG_FILE,
    }) {
        Ok(opts) => opts,
        Err(err) => {
            return log::error!("{err}");
        }
    };

    if let Err(err) = match command {
        EasexxCommand::Build => commands::build::exec(&options),
        EasexxCommand::Test => commands::test::exec(&options),
        EasexxCommand::Man => commands::man::exec(),
    } {
        log::error!("{err}");
    }
}
