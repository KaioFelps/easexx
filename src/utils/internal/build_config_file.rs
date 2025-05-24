const BUILD_FILE_PATH_FLAG: &str = "-p";

pub fn get_build_config_from_flags(args: &[String]) -> Option<String> {
    let flag_index = args
        .iter()
        .position(|arg| arg.starts_with(BUILD_FILE_PATH_FLAG))?;

    let flag = &args[flag_index];

    if flag.contains("=") {
        let config_file = flag
            .split("=")
            .collect::<Vec<_>>()
            .get(1)
            .map(ToString::to_string)?;

        if config_file.is_empty() {
            return None;
        }

        return Some(config_file.to_owned());
    }

    let adjacent_arg = args.get(flag_index + 1)?;

    if adjacent_arg.is_empty() {
        return None;
    }

    Some(adjacent_arg.to_owned())
}
