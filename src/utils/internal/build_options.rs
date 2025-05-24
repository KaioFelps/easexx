use std::{fs::File, io::ErrorKind};

use crate::{build_options::BuildOptions, utils::get_build_config_from_flags};

pub struct GetBuildOptionsArgs<'a> {
    pub args: &'a [String],
    pub default_config_file_path: &'a str,
}

pub fn get_build_options(
    GetBuildOptionsArgs {
        args,
        default_config_file_path,
    }: GetBuildOptionsArgs<'_>,
) -> anyhow::Result<BuildOptions> {
    let config_file =
        get_build_config_from_flags(args).unwrap_or_else(|| default_config_file_path.to_string());

    let config_file = match File::open(&config_file) {
        Ok(file) => file,
        Err(err) => {
            if err.kind() == ErrorKind::NotFound {
                return Err(anyhow::Error::msg(format!(
                    "Could not find \"{config_file}\". Try creating it or using \"-p\" flag \
                        to specify the path to the build config file."
                )));
            }

            return Err(err.into());
        }
    };

    BuildOptions::read_from_config_file(config_file)
}
