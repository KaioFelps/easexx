use std::{io, path::Path, process::Command};

use crate::{
    build_options::BuildOptions,
    common::{to_flag_vector, CompilerFlag},
};

pub struct BuildSourceFileArgs<'a> {
    pub source_file: &'a str,
    pub output_dir: Option<&'a str>,
    pub options: &'a BuildOptions,
    pub output_buffer: &'a mut Vec<String>,
}

pub fn build_source_file(
    BuildSourceFileArgs {
        options,
        output_buffer,
        output_dir,
        source_file,
    }: BuildSourceFileArgs<'_>,
) -> io::Result<()> {
    let output_file = format!(
        "{}/{}",
        output_dir.unwrap_or(options.build_dir.as_str()),
        get_output_file_from_source_file(source_file)
    );

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

    match compile_command.spawn() {
        Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err)),
        Ok(mut child) => {
            let _ = child.wait()?;
            output_buffer.push(output_file);
        }
    }

    Ok(())
}

fn get_output_file_from_source_file(path: &str) -> String {
    Path::new(path)
        .with_extension("o")
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}
