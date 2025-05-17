use crate::common::{
    build_source_file, discover_cpp_files, link_all_objects, maybe_create_dir, BuildSourceFileArgs,
    LinkAllObjectsArgs,
};
use crate::BuildOptions;
use std::io;
use std::path::Path;
use std::process::Command;

pub fn exec(options: &BuildOptions) -> io::Result<()> {
    let tests_build_dir = format!("{}/__tests__", &options.build_dir);
    maybe_create_dir(&tests_build_dir);

    let src_compiled_objects = super::build::compile_without_linking_from_src_dir(options)?
        .into_iter()
        .filter(|file| !file.contains("/main."))
        .collect::<Vec<_>>();

    let tests_dir: &str = "tests";
    let tests_dir = std::fs::read_dir(tests_dir).expect("Could not find src directory");

    let mut source_files = Vec::new();
    tests_dir.for_each(|entry| discover_cpp_files(entry, &mut source_files));

    let mut compiled_objects = Vec::new();

    for ref source_file in source_files {
        build_source_file(BuildSourceFileArgs {
            options,
            output_buffer: &mut compiled_objects,
            source_file,
            output_dir: Some(&tests_build_dir),
        })?;
    }

    let (test_executable_files, test_resource_files): (Vec<String>, Vec<String>) = compiled_objects
        .into_iter()
        .partition(|file| file.contains("_test."));

    let tests_executables = test_executable_files
        .iter()
        .map(|executable_object| {
            let executable_file = get_filename_without_extension(executable_object);

            let executable_path = link_all_objects(LinkAllObjectsArgs {
                options,
                output_dir: Some(&tests_build_dir),
                output_files: &[
                    src_compiled_objects.as_slice(),
                    test_resource_files.as_slice(),
                    &[executable_object.to_owned()],
                ]
                .concat(),
                include_dev_libs: true,
                output_filename: Some(&executable_file),
            });

            executable_path
        })
        .collect::<Vec<_>>();

    tests_executables.iter().for_each(run_executable);

    Ok(())
}

fn run_executable(executable: &String) {
    let _ = Command::new(executable).status();
}

fn get_filename_without_extension(path: &str) -> String {
    Path::new(path)
        .file_name()
        .unwrap()
        .to_string_lossy()
        .strip_suffix(".o")
        .unwrap()
        .to_string()
}
