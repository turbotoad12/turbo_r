/*
Prints a string with a newline;
 */
pub fn print_str(s: &str) {
    println!("{}", s);
}

pub mod git {
    use git2::Repository;
    pub fn git_clone(url: &str, dst: &str) -> Result<(), git2::Error> {
        Repository::clone(url, dst)?;
        Ok(())
    }
}