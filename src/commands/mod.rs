use std::fmt::Display;

pub mod build;
pub mod man;
pub mod test;

pub enum EasexxCommand {
    Man,
    Build,
    Test,
}

impl Display for EasexxCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Build => "build",
                Self::Man => "man",
                Self::Test => "test",
            }
        )
    }
}

impl TryFrom<&str> for EasexxCommand {
    type Error = std::io::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "build" => Ok(Self::Build),
            "test" => Ok(Self::Test),
            "man" => Ok(Self::Man),
            _ => Err(std::io::Error::other(
                "No valid command has been found. Try \"man\" command \
                for instructions on how to use this CLI.",
            )),
        }
    }
}

impl TryFrom<String> for EasexxCommand {
    type Error = std::io::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        EasexxCommand::try_from(value.as_str())
    }
}
