/*
Prints a string with a newline;
 */



pub fn print_str(s: &str) {
    println!("{}", s);
}

pub mod fs {
    use std::fs;
    use std::io;
    use std::path::Path;
    pub fn delete_dir_recursive(dir: &str) {

        if let Err(e) = fs::remove_dir_all(dir) {
            eprintln!("Failed to remove directory '{}': {}", dir, e);
        }
    }
    
    /// Lists all files (not directories) in the specified directory.
    /// Returns a Vec<String> of file names, or an io::Error if something goes wrong.
    fn list_files_in_dir<P: AsRef<Path>>(dir: P) -> io::Result<Vec<String>> {
        let mut file_names = Vec::new();
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    file_names.push(name.to_string());
                }
            }
        }
        Ok(file_names)
    }

}