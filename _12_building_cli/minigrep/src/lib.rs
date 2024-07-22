use std::{env, error::Error, fs};

pub struct Config {
  pub query: String,
  pub file_path: String,
  pub ignore_case: bool,
}
impl Config {
  pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
      args.next(); // 첫번째 버림

      let query = match args.next() {
        Some(x) => x,
        None => return Err("Didn't get a query string(first argument)"),
      };
      let file_path = match args.next() {
        Some(x) => x,
        None => return Err("Didn't get a file_path(second argument)"),
      };

      let ignore_case = env::var("IGNORE_CASE").is_ok();
  
      Ok(Self{ query, file_path, ignore_case })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.file_path)?;

  if config.ignore_case {
    search_case_insensitive(&config.query, &contents).iter().for_each(|line| println!("{line}"));
  } else {
    search(&config.query, &contents).iter().for_each(|line| println!("{line}"));
  };
  
  Ok(())
}

pub fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
  contents.lines().filter(|line| line.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let query = query.to_uppercase();
  contents.lines().filter(|line| line.to_uppercase().contains(&query)).collect()
}

#[cfg(test)]
mod tests {
  use std::vec;

use super::*;

  #[test]
  fn one_result() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }
  
  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fas,t productive.
Pick three.
Trust me.";

    assert_eq!(
      vec!["Rust:", "Trust me."],
      search_case_insensitive(query, contents)
    )
  }
}