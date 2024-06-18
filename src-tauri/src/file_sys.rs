use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::Path;

// TODO extend this to optionally not include the entire file structure
pub fn save_bytes_to_file(bytes: &[u8], save_path: &str) -> Result<(), Box<dyn Error>>{
    let path_to_save = Path::new(save_path);
    // TODO check all of these for Errors for production
    let prefix = path_to_save.parent().unwrap();
    fs::create_dir_all(prefix).unwrap();
    let mut new_file = File::create(path_to_save).unwrap();
    new_file.write_all(bytes).unwrap();

    let _ = new_file.flush();
    drop(new_file);

    Ok(())
}

// kind of unnessesary logic wise but there so that only one script handles the fs interaction
pub fn load_file_as_string(file_path: &str) -> Result<String, Box<dyn Error>>{
    let contents: string = fs::read_to_string(file_path).expect("If it crashes here there is an error on the js file selection code");
    Ok(contents)
}
