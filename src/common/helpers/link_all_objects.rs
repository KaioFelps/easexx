use super::resolve_output_file::{
    resolve_output_file_path, OutputFileExtension, ResolveOutputFileArgs,
};
use crate::build_options::BuildOptions;
use crate::common::{to_flag_vector, CompilerFlag};
use std::process::Command;

pub struct LinkAllObjectsArgs<'a> {
    pub output_files: &'a Vec<String>,
    pub output_dir: Option<&'a str>,
    pub options: &'a BuildOptions,
    pub include_dev_libs: bool,
    pub output_filename: Option<&'a str>,
}

pub fn link_all_objects(link_all_objects_args: LinkAllObjectsArgs<'_>) -> String {
    let (output_dir, output_filename) = resolve_output_dir_and_filename(&link_all_objects_args);
    let executable = get_executable(output_dir, output_filename);

    let mut link_command = prepare_obj_linkage_command(link_all_objects_args, &executable);

    match link_command.spawn() {
        Err(err) => {
            panic!("Failed to link compiled binaries into executable. {err}")
        }
        Ok(mut child) => {
            let _ = child.wait();
            executable
        }
    }
}

fn resolve_output_dir_and_filename<'a>(
    link_all_objects_args: &'a LinkAllObjectsArgs<'a>,
) -> (&'a str, &'a str) {
    (
        link_all_objects_args
            .output_dir
            .unwrap_or(&link_all_objects_args.options.build_dir),
        link_all_objects_args
            .output_filename
            .unwrap_or(&link_all_objects_args.options.output_filename),
    )
}

fn get_executable(output_dir: &str, source_file: &str) -> String {
    #[cfg(not(windows))]
    let extension = OutputFileExtension::UnixExecutable;
    #[cfg(windows)]
    let extension = OutputFileExtension::WindowsExecutable;

    resolve_output_file_path(ResolveOutputFileArgs {
        extension,
        output_dir,
        source_file,
    })
}

fn prepare_obj_linkage_command(
    LinkAllObjectsArgs {
        options,
        output_files,
        include_dev_libs,
        ..
    }: LinkAllObjectsArgs<'_>,
    executable: &str,
) -> Command {
    let mut compiler_command_args = options.compiler.split(" ").collect::<Vec<_>>();
    let compiler_program = compiler_command_args.remove(0);

    let mut link_command = Command::new(compiler_program);
    link_command
        .args(compiler_command_args)
        .args(&options.compiler_flags)
        .args(to_flag_vector(&options.include_dirs, CompilerFlag::Include));

    link_command.args(to_flag_vector(&options.lib_dirs, CompilerFlag::LibDir));
    if include_dev_libs {
        link_command.args(to_flag_vector(&options.dev_lib_dirs, CompilerFlag::LibDir));
    }

    link_command
        .args(output_files)
        .args(to_flag_vector(&options.libs, CompilerFlag::Lib));
    if include_dev_libs {
        link_command.args(to_flag_vector(&options.dev_libs, CompilerFlag::Lib));
    }

    link_command.arg("-o").arg(executable);

    link_command
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::{
        build_options::BuildOptions,
        test_utils::{fixtures::build_options, helpers::stringify_command},
    };

    use super::{prepare_obj_linkage_command, LinkAllObjectsArgs};

    #[rstest]
    #[test]
    fn should_correctly_format_the_compiler_command(build_options: BuildOptions) {
        let command = prepare_obj_linkage_command(
            LinkAllObjectsArgs {
                include_dev_libs: true,
                options: &build_options,
                output_dir: None,
                output_filename: None,
                output_files: &vec!["foo.o".into()],
            },
            "foo",
        );

        let command_as_str = stringify_command(&command);

        assert_eq!(
            "zig c++ -std=c++20 -Iinclude -Lbin/libs foo.o -lCatch2 -o foo",
            command_as_str
        );
    }
}
