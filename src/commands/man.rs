struct ManualCommandFlag<'a> {
    aliases: Vec<&'a str>,
    description: &'a str,
    example: &'a str,
}

struct ManualCommand<'a> {
    command: &'a str,
    description: &'a str,
    flags: Vec<ManualCommandFlag<'a>>,
}

pub fn exec() -> anyhow::Result<()> {
    let commands: Vec<ManualCommand> = vec![
            ManualCommand {
                command: "build",
                description: "Build the current application using data from `./build.json`.",
                flags: vec![
                    ManualCommandFlag {
                        aliases: vec!["-p"],
                        description: "Tells the path to the `build.json` file.",
                        example: "./easexx build -p=path/to/build.json"
                    }
                ]
            },
            ManualCommand {
                command: "test",
                description: "Build the whole application and files from `tests` directory and execute each \"*_test.cpp\" file.",
                flags: vec![]
            }
        ];

    println!("This is the cli tool for building this C++ application.");
    println!("Available commands are:");

    for ManualCommand {
        command,
        description,
        flags,
    } in commands
    {
        println!("\"{command}\": {description}");

        if !flags.is_empty() {
            println!("    Flags:");
        }

        for ManualCommandFlag {
            aliases,
            description,
            example,
        } in flags
        {
            println!(
                "        Aliases: {}\n        \
                {description}\n        \
                Example: {example}\n",
                aliases.join(", ")
            );
        }
    }

    Ok(())
}
