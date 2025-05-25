use std::{fs::File, io::Error, time::SystemTime};

pub struct ShouldRecompileArgs<'a> {
    pub src_filename: &'a str,
    pub object_filename: &'a str,
}

pub fn should_recompile(
    ShouldRecompileArgs {
        src_filename,
        object_filename,
    }: ShouldRecompileArgs<'_>,
) -> bool {
    let src_file = match File::open(src_filename) {
        Err(_) => return true,
        Ok(file) => file,
    };

    let object_file = match File::open(object_filename) {
        Err(_) => return true,
        Ok(file) => file,
    };

    let file_modified_time = match get_modified_time(&src_file) {
        Err(_) => return true,
        Ok(time) => time,
    };

    let obj_file_modified_time = match get_modified_time(&object_file) {
        Err(_) => return true,
        Ok(time) => time,
    };

    file_modified_time > obj_file_modified_time
}

fn get_modified_time(file: &File) -> Result<SystemTime, Error> {
    file.metadata()?.modified()
}
