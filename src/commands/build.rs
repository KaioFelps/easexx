use crate::common::{
    build_source_file, discover_cpp_files, link_all_objects, maybe_create_dir, BuildSourceFileArgs,
    LinkAllObjectsArgs,
};
use crate::{BuildOptions, SOURCE_DIR};
use std::io;

pub fn exec(options: &BuildOptions) -> io::Result<()> {
    let compiled_objects = match compile_without_linking_from_src_dir(SOURCE_DIR, options)? {
        Some(objects) => objects,
        None => {
            log::warn!("{SOURCE_DIR} directory not found. Skipping build command.");
            return Ok(());
        }
    };

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
    src_dir: &str,
    options: &BuildOptions,
) -> io::Result<Option<Vec<String>>> {
    maybe_create_dir(&options.build_dir);

    let exists = std::fs::exists(src_dir);
    if !exists.is_ok_and(|res| res) {
        return Ok(None);
    }

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

    Ok(Some(compiled_objects))
}
