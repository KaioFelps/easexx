use std::process::Command;

use crate::{
    build_options::BuildOptions,
    common::{to_flag_vector, CompilerFlag},
};

pub struct LinkAllObjectsArgs<'a> {
    pub output_files: &'a Vec<String>,
    pub output_dir: Option<&'a str>,
    pub options: &'a BuildOptions,
    pub include_dev_libs: bool,
    pub output_filename: Option<&'a str>,
}

pub fn link_all_objects(
    LinkAllObjectsArgs {
        options,
        output_dir,
        output_files,
        include_dev_libs,
        output_filename,
    }: LinkAllObjectsArgs<'_>,
) -> String {
    let mut compiler_command_args = options.compiler.split(" ").collect::<Vec<_>>();
    let compiler_program = compiler_command_args.remove(0);

    #[allow(unused_mut)]
    let mut executable = output_filename
        .unwrap_or(options.output_file_name.as_str())
        .to_string();

    #[cfg(windows)]
    executable.push_str(".exe");

    let executable = format!(
        "{}/{}",
        output_dir.unwrap_or(options.build_dir.as_str()),
        executable
    );

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

    link_command.arg("-o").arg(&executable);

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
