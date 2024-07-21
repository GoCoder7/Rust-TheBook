use std::{error::Error, fs};

pub struct Config {
  pub query: String,
  pub file_path: String,
}
impl Config {
  pub fn build(args: &[String]) -> Result<Self, &str> {
      if args.len() < 3 {
          return Err("not enough arguments");
      }

      let query = args[1].to_owned();
      let file_path = args[2].to_owned();
  
      Ok(Self{ query, file_path })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.file_path)?;
  println!("With text:\n{}",&contents);
  Ok(())
}