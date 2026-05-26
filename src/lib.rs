/*
Prints a string with a newline;
 */



pub fn print_str(s: &str) {
    println!("{}", s);
}

pub mod fs {
    use std::fs;
    pub fn delete_dir_recursive(dir: &str) {

        if let Err(e) = fs::remove_dir_all(dir) {
            eprintln!("Failed to remove directory '{}': {}", dir, e);
        }
    }
}