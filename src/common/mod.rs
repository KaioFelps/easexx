use std::fmt::Display;
use std::fs::DirEntry;
pub mod helpers;

pub use helpers::build_source_file::*;
pub use helpers::link_all_objects::*;

pub enum CompilerFlag {
    Include,
    LibDir,
    Lib,
}

impl Display for CompilerFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Self::Include => "-I",
                Self::LibDir => "-L",
                Self::Lib => "-l",
            }
        )
    }
}

pub fn to_flag_vector(content: &[String], flag: CompilerFlag) -> Vec<String> {
    content
        .iter()
        .map(|content| format!("{flag}{content}"))
        .collect()
}

pub fn maybe_create_dir(dir: &str) {
    std::fs::DirBuilder::new()
        .recursive(true)
        .create(dir)
        .expect("Could not create directory {dir}.");
}

pub fn discover_cpp_files(entry: Result<DirEntry, std::io::Error>, buffer: &mut Vec<String>) {
    let entry = match entry {
        Ok(entry) => entry,
        Err(err) => {
            println!("Failed to open entry {err}");
            return;
        }
    };

    let file_type = match entry.file_type() {
        Ok(entry) => entry,
        Err(_) => return,
    };

    if file_type.is_dir() {
        match std::fs::read_dir(entry.path()) {
            Err(err) => println!("Failed to read subdirectory with err: {err}",),
            Ok(reader) => {
                reader.for_each(|entry| discover_cpp_files(entry, buffer));
            }
        }
        return;
    }

    if file_type.is_file()
        && entry
            .path()
            .extension()
            .is_some_and(|extension| extension == "cpp")
    {
        buffer.push(
            entry
                .path()
                .to_str()
                .expect("Encountered source file with invalid path.")
                .replace("\\", "/"),
        );
    }
}

#[cfg(test)]
mod test {
    use crate::common::CompilerFlag;

    use super::to_flag_vector;

    #[test]
    fn test_to_flag_vector() {
        assert_eq!(
            vec![
                "-Lvcpkg_installed/x64-windows/lib".to_string(),
                "-Lvcpkg_installed/x64-windows/lib/manual-link".to_string()
            ],
            to_flag_vector(
                &[
                    "vcpkg_installed/x64-windows/lib".into(),
                    "vcpkg_installed/x64-windows/lib/manual-link".into(),
                ],
                CompilerFlag::LibDir
            )
        );

        assert!(to_flag_vector(&[], CompilerFlag::LibDir).is_empty());
    }
}
