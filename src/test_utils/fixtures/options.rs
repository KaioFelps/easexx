use rstest::fixture;

use crate::{
    build_options::BuildOptions,
    utils::{get_build_options, GetBuildOptionsArgs},
};

#[fixture]
pub fn build_options() -> BuildOptions {
    get_build_options(GetBuildOptionsArgs {
        args: &["-p=tests/resources/build.json".to_string()],
        default_config_file_path: "tests/resources/build",
    })
    .unwrap()
}
