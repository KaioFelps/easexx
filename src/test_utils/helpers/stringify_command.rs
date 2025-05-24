use std::process::Command;

pub fn stringify_command(command: &Command) -> String {
    let mut command_as_str = command.get_program().to_string_lossy().to_string();

    let args = command
        .get_args()
        .map(|arg| arg.to_string_lossy())
        .collect::<Vec<_>>()
        .join(" ");

    command_as_str.push(' ');
    command_as_str.push_str(&args);

    command_as_str
}
