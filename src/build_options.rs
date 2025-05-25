use std::fs::File;

use serde::Deserialize;

#[derive(Deserialize)]
struct PartialBuildOptions {
    pub compiler: String,

    #[serde(rename = "compilerFlags")]
    pub compiler_flags: Option<Vec<String>>,

    #[serde(rename = "includeDirs")]
    pub include_dirs: Option<Vec<String>>,

    #[serde(rename = "libDirs")]
    pub lib_dirs: Option<Vec<String>>,
    pub libs: Option<Vec<String>>,

    #[serde(rename = "devLibDirs")]
    pub dev_lib_dirs: Option<Vec<String>>,
    #[serde(rename = "devLibs")]
    pub dev_libs: Option<Vec<String>>,

    #[serde(rename = "buildDir")]
    pub build_dir: Option<String>,

    #[serde(rename = "outputFileName")]
    pub output_filename: String,
}

pub struct BuildOptions {
    pub compiler: String,
    pub compiler_flags: Vec<String>,
    pub include_dirs: Vec<String>,
    pub lib_dirs: Vec<String>,
    pub libs: Vec<String>,
    pub dev_lib_dirs: Vec<String>,
    pub dev_libs: Vec<String>,
    pub build_dir: String,
    pub output_filename: String,
    pub release: bool,
}

impl BuildOptions {
    pub fn read_from_config_file(file: File) -> anyhow::Result<Self> {
        let options: PartialBuildOptions = serde_json::from_reader(file).map_err(|err| {
            anyhow::Error::msg(format!("Failed to parse build config file: {err}"))
        })?;

        Ok(options.into())
    }
}

impl From<PartialBuildOptions> for BuildOptions {
    fn from(value: PartialBuildOptions) -> Self {
        BuildOptions {
            build_dir: value.build_dir.unwrap_or("build".into()),
            compiler: value.compiler,
            compiler_flags: value.compiler_flags.unwrap_or_default(),
            dev_lib_dirs: value.dev_lib_dirs.unwrap_or_default(),
            dev_libs: value.dev_libs.unwrap_or_default(),
            include_dirs: value.include_dirs.unwrap_or_default(),
            lib_dirs: value.lib_dirs.unwrap_or_default(),
            libs: value.libs.unwrap_or_default(),
            output_filename: value.output_filename,
            release: std::env::args().any(|arg| arg == "--release"),
        }
    }
}
