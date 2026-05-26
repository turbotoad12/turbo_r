/*
Prints a string with a newline;
 */



pub fn print_str(s: &str) {
    println!("{}", s);
}

#[deprecated]
pub mod git {
    use git2::Repository;
    pub fn git_clone(url: &str, dst: &str) -> Result<(), git2::Error> {
        Repository::clone(url, dst)?;
        Ok(())
    }
}

pub mod fs {
    use std::fs;
    pub fn delete_dir_recursive(dir: &str) {

        if let Err(e) = fs::remove_dir_all(dir) {
            eprintln!("Failed to remove directory '{}': {}", dir, e);
        }
    }
}