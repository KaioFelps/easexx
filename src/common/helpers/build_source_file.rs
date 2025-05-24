use std::process::Command;

use crate::{
    build_options::BuildOptions,
    common::{to_flag_vector, CompilerFlag},
};

pub struct BuildSourceFileArgs<'a> {
    pub source_file: &'a str,
    pub options: &'a BuildOptions,
    pub output_buffer: &'a mut Vec<String>,
    pub output_file: String,
}

pub fn build_source_file(mut build_source_file_args: BuildSourceFileArgs) -> anyhow::Result<()> {
    let mut compile_command = prepare_compile_command(&mut build_source_file_args)?;
    let mut child = compile_command.spawn()?;
    let _ = child.wait()?;

    build_source_file_args
        .output_buffer
        .push(build_source_file_args.output_file);

    Ok(())
}

fn prepare_compile_command(
    BuildSourceFileArgs {
        options,
        source_file,
        output_file,
        ..
    }: &mut BuildSourceFileArgs<'_>,
) -> anyhow::Result<Command> {
    let mut compiler_vec = options
        .compiler
        .split(" ")
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    let compiler = compiler_vec.remove(0);

    let mut compile_command = Command::new(compiler);
    compile_command
        .args(compiler_vec)
        .args(options.compiler_flags.as_slice())
        .args(to_flag_vector(&options.include_dirs, CompilerFlag::Include))
        .arg("-c")
        .arg(source_file)
        .arg("-o")
        .arg(&output_file);

    Ok(compile_command)
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::{
        build_options::BuildOptions,
        common::{helpers::build_source_file::prepare_compile_command, BuildSourceFileArgs},
        test_utils::{fixtures::build_options, helpers::stringify_command},
    };

    #[rstest]
    #[test]
    fn it_should_correctly_prepare_the_compile_command(build_options: BuildOptions) {
        let mut output_buffer = Vec::new();

        let compile_command = prepare_compile_command(&mut BuildSourceFileArgs {
            options: &build_options,
            output_buffer: &mut output_buffer,
            output_file: "foo.o".into(),
            source_file: "foo.cpp",
        })
        .unwrap();

        let compile_command = stringify_command(&compile_command);

        assert_eq!(
            "zig c++ -std=c++20 -Iinclude -c foo.cpp -o foo.o",
            compile_command
        );
    }
}
