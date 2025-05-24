use std::{fmt::Display, path::Path};

pub enum OutputFileExtension {
    #[allow(unused)]
    WindowsExecutable,

    #[allow(unused)]
    UnixExecutable,

    Object,
}

impl Display for OutputFileExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Object => "o",
                Self::UnixExecutable => "",
                Self::WindowsExecutable => "exe",
            }
        )
    }
}

pub struct ResolveOutputFileArgs<'a> {
    pub output_dir: &'a str,
    pub source_file: &'a str,
    pub extension: OutputFileExtension,
}

pub fn resolve_output_file_path(
    ResolveOutputFileArgs {
        output_dir,
        source_file,
        extension,
    }: ResolveOutputFileArgs<'_>,
) -> String {
    let output_dir = output_dir.replace("\\", "/");
    let output_dir_without_trailing_slash = output_dir.strip_suffix("/").unwrap_or(&output_dir);

    format!(
        "{}/{}",
        output_dir_without_trailing_slash,
        get_output_filename_from_source_file(source_file, &extension.to_string())
    )
}

fn get_output_filename_from_source_file(path: &str, extension: &str) -> String {
    Path::new(path)
        .with_extension(extension)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::common::helpers::resolve_output_file::OutputFileExtension;

    use super::{
        get_output_filename_from_source_file, resolve_output_file_path, ResolveOutputFileArgs,
    };

    #[rstest]
    #[case("", "foo")]
    #[case("o", "foo.o")]
    #[case("exe", "foo.exe")]
    #[test]
    fn should_extract_filename_with_object_extension(
        #[case] extension: &str,
        #[case] result: &str,
    ) {
        let output_filename =
            get_output_filename_from_source_file("tests/resources/foo.cpp", extension);
        assert_eq!(result, output_filename);
    }

    #[rstest]
    #[test]
    #[case("tests/resources/build")]
    #[case("tests/resources/build/")]
    fn should_correctly_resolve_output_file_path(#[case] output_dir: &str) {
        let output_file_path = resolve_output_file_path(ResolveOutputFileArgs {
            output_dir,
            source_file: "tests/resources/foo.cpp",
            extension: OutputFileExtension::Object,
        });

        assert_eq!("tests/resources/build/foo.o", output_file_path);
    }
}
