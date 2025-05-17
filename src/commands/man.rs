use std::{io, sync::OnceLock};

static AVAILABLE_COMMANDS: OnceLock<Vec<[&str; 2]>> = OnceLock::new();

pub fn exec() -> io::Result<()> {
    let commands = AVAILABLE_COMMANDS.get_or_init(|| {
        vec![[
            "build",
            "Build the current application using data from `./build.json`.",
        ], [
            "test",
            "Build the whole application and files from `tests` directory and execute each \"*_test.cpp\" file."
        ]]
    });

    println!("This is the cli tool for building this C++ application.");
    println!("Available commands are:");

    for [command, title] in commands {
        println!("{command:10}\t{title}");
    }

    Ok(())
}
