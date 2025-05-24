use std::env;

use crate::commands::EasexxCommand;

pub fn get_command_from_args() -> EasexxCommand {
    env::args()
        .find(|command| EasexxCommand::try_from(command.as_str()).is_ok())
        .map(|command| EasexxCommand::try_from(command).unwrap())
        .unwrap_or_else(|| {
            log::warn!("Nenhum comando válido foi encontrado.");
            EasexxCommand::Man
        })
}
