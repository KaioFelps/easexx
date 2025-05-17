use crate::common::{
    build_source_file, discover_cpp_files, link_all_objects, maybe_create_dir, BuildSourceFileArgs,
    LinkAllObjectsArgs,
};
use crate::BuildOptions;
use std::io;

pub fn exec(options: &BuildOptions) -> io::Result<()> {
    let compiled_objects = compile_without_linking_from_src_dir(options)?;

    let executable_file_name = link_all_objects(LinkAllObjectsArgs {
        options,
        output_dir: None,
        output_files: &compiled_objects,
        include_dev_libs: !options.release,
        output_filename: None,
    });

    println!("Compilado com sucesso! Rode utilizando \"./{executable_file_name}\"");

    Ok(())
}

pub(super) fn compile_without_linking_from_src_dir(
    options: &BuildOptions,
) -> io::Result<Vec<String>> {
    maybe_create_dir(&options.build_dir);

    let src_dir: &str = "src";
    let src_dir =
        std::fs::read_dir(src_dir).unwrap_or_else(|_| panic!("Could not find {src_dir} directory"));

    let mut source_files = Vec::new();
    src_dir.for_each(|entry| discover_cpp_files(entry, &mut source_files));

    let mut compiled_objects = Vec::new();

    for ref source_file in source_files {
        build_source_file(BuildSourceFileArgs {
            options,
            output_buffer: &mut compiled_objects,
            source_file,
            output_dir: None,
        })?;
    }

    Ok(compiled_objects)
}
