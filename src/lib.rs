use std::fmt;
use std::error::Error;
use std::fs;

pub struct Config{
    pub ignore_case: bool,
    pub query: String,
    pub file_path: String,
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

        let ignore_case = match args.next() {
            Some(arg) => arg == "--ignore-case",
            None => false,
        };

        let query = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),

        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };



        Ok(Config{ignore_case, query, file_path})
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

    let results = search(
        &config.query,
        &fs::read_to_string(config.file_path)?,
        Some(config.ignore_case));

    for line in results{
        println!("Line {} = {}", line.line_number, line.line);
    }

    Ok(())
}

pub fn search(query: &str, contents: &str, config: Option<bool>) -> Vec<Output>{

    let config = config.unwrap_or(false);
    let mut pattern = String::from(query);
    if config{
        pattern = query.to_lowercase();
    }

    let results:Vec<_> = contents.lines()
                .enumerate()
                .filter(|(_ ,line)| line.to_lowercase().contains(&pattern))
                .collect();

    let mut output: Vec<Output> =  Vec::new();

    for (i, line) in results{
        output.push(Output::new(i, String::from(line)));
    }
    output
}

#[cfg(test)]
mod tests{
    use std::vec;
    use super::*;

    #[test]
    fn one_result(){
        let query = "rod";
        let contents = "
Rust:
safe, fast, productive.
Pick three.
Duct Tape.";

        assert_eq!(
            vec![Output::new(2, String::from("safe, fast, productive."))],
            search(query, contents, Some(false)));
        }

    #[test]
    fn case_insensitive(){
        let query = "rUSt";
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
            search(query, contents, Some(true)));
    }

}