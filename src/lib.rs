use std::fmt;
use std::error::Error;
use std::{env, fs};

pub struct Config{
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Output{
    pub line_number: usize,
    pub line: String,
}

impl Config{
    /// Builds a new Config instance from the command-line arguments.
    ///
    /// # Arguments
    ///
    /// * `args` - An iterator over the command-line arguments.
    ///
    /// # Errors
    ///
    /// Returns an error if the arguments cannot be parsed.
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str>{
        args.next();// skip the first argument which is the name of the program

        let query = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),

        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE CASE").is_ok();

        Ok(Config{query, file_path, ignore_case})
    }
}

impl Output{
    pub fn new(line_number: usize, line: String) -> Self{
        Output{line_number, line}
    }
}
impl fmt::Display for Output{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}: {}", self.line_number, self.line)
    }
}

/// Runs the main functionality of the frep program.
///
/// # Arguments
///
/// * `config` - The configuration for the program.
///
/// # Errors
///
/// Returns an error if the program cannot complete successfully.

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case{
        search_case_insensitive(&config.query, &contents)
    }   else {
        search(&config.query, &contents)
    };

    for line in results{
        println!("Result = \"{line}\"")
    }

    Ok(())
}

pub fn search(query: &str, contents: &str) -> Vec<Output>{
    let results:Vec<_> = contents.lines()
                .enumerate()
                .filter(|(_ ,line)| line.contains(query))
                .collect();

    let mut output: Vec<Output> =  Vec::new();

    for (i, line) in results{
        output.push(Output::new(i, String::from(line)));
    }
    output
}

pub fn search_case_insensitive(query: &str, contents: &str) -> Vec<Output>{
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for (i, line) in contents.lines().enumerate(){
        if line.to_lowercase().contains(&query){
            results.push(Output::new(i-1, String::from(line)));
        }
    }
    results
}

#[cfg(test)]
mod tests{
    use std::vec;
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "
Rust:
safe, fast, productive.
Pick three.
Duct Tape.";

        assert_eq!(
            vec![Output::new(2, String::from("safe, fast, productive."))],
            search(query, contents));
        }

    #[test]
    fn case_insensitive(){
        let query = "rUsT";
        let contents = "

Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec![
                Output::new(1, String::from("Rust:")),
                Output::new(4, String::from("Trust me."))
            ],
            search_case_insensitive(query, contents));
    }

}