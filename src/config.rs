extern crate rustc_serialize;

use rustc_serialize::json;
use std::path::{PathBuf, Path};
use std::env::{current_dir, home_dir};
use std::fs::File;
use std::io::Read;

#[derive(Debug, RustcDecodable)]
pub struct Args {
    pub flag_dry_run: bool,
    pub cmd_list: bool,
    pub cmd_push: bool,
    pub flag_config: String,
}

#[derive(Debug, PartialEq)]
pub struct Error {
  message: String,
}

#[derive(Debug, PartialEq, RustcDecodable)]
struct ConfigFromDisk {
  observed_folders: Vec<String>,
}

impl ConfigFromDisk {
  fn read(config_path : &Path) -> Result<ConfigFromDisk, Error> {
    let content : String = try!(ConfigFromDisk::read_file(config_path));
    match json::decode(&content) {
      Ok(decoded) => Ok(decoded),
      Err(_) => {
        let msg = format!("The file '{}' contained invalid json", config_path.to_str().unwrap_or(""));
        Err(Error { message: msg })
      }
    }
  }

  fn read_file(config_path : &Path) -> Result<String, Error> {
    let mut result : String = String::new();
    let msg = format!("Could not read the file '{}'", config_path.to_str().unwrap_or(""));
    let error = Error { message: msg };
    match File::open(config_path) {
      Ok(mut file) => {
        match file.read_to_string(&mut result) {
          Ok(_) => Ok(result),
          Err(_) => Err(error)
        }
      },
      Err(_) => Err(error)
    }
  }
}

#[derive(Debug, PartialEq)]
pub struct CliOptions {
    dry_run: bool,
    observed_folders: Vec<PathBuf>,
}

impl CliOptions {
  pub fn from(args: &Args) -> Result<CliOptions, Error> {
    let config_path = CliOptions::absolute_path(&args.flag_config);
    let config = try!(ConfigFromDisk::read(config_path.as_path()));
    let folders : Vec<String> = config.observed_folders;
    let observed_folders : Vec<PathBuf> = folders.iter().map(|folder| PathBuf::from(folder)).collect::<Vec<_>>();
    Ok(CliOptions {
      dry_run: args.flag_dry_run,
      observed_folders: observed_folders,
    })
  }

  fn absolute_path(path : &String) -> PathBuf {
    //If the two unwrap() operations fail then we might as well give up
    let input_path = PathBuf::from(path);
    if input_path.has_root() {
      input_path
    } else if input_path.starts_with("~") {
      let mut absolute_path = home_dir().unwrap();
      let everything_after_tilde = input_path.iter().skip(1);
      for component in everything_after_tilde {
        absolute_path.push(component);
      }
      absolute_path
    } else {
      let mut absolute_path = current_dir().unwrap();
      absolute_path.push(input_path);
      absolute_path
    }
  }
}

#[cfg(test)]
mod tests {
    use std::io::prelude::*;
    use std::fs;
    use std::fs::File;
    use std::env::{current_dir, home_dir};
    use std::path::{Path, PathBuf};
    use super::{ConfigFromDisk, Error, Args, CliOptions};

    #[test]
    fn test_config_from_file_read_with_existing_file() {
      let config_path = "sandbox1/projects.json";
      let json = "{\"observed_folders\": [\"projects\", \"code\"]}";
      write_json_to(json, config_path);
      let actual = ConfigFromDisk::read(Path::new(config_path));
      let expected = ConfigFromDisk {
        observed_folders: vec![String::from("projects"), String::from("code")]
      };
      assert_eq!(actual, Ok(expected));
      clean_sandbox(config_path);
    }

    #[test]
    fn test_config_from_file_read_with_missing_file() {
      let config_path = "sandbox2/nothing.json";
      clean_sandbox(config_path);
      let actual = ConfigFromDisk::read(Path::new(config_path));
      assert_eq!(actual, Err(Error::from("Could not read the file 'sandbox2/nothing.json'")));
    }

    #[test]
    fn test_config_from_file_read_with_invalid_json() {
      let config_path = "sandbox3/invalid.json";
      let json = "{\"observed_folder\": [\"projects\", \"code\"]}";
      write_json_to(json, config_path);
      let actual = ConfigFromDisk::read(Path::new(config_path));
      assert_eq!(actual, Err(Error::from("The file 'sandbox3/invalid.json' contained invalid json")));
      clean_sandbox(config_path);
    }

    #[test]
    fn test_cli_options_from_with_existing_file() {
      let config_path = "sandbox4/projects.json";
      let json = "{\"observed_folders\": [\"projects\", \"code\"]}";
      write_json_to(json, config_path);
      let args = Args {
        flag_dry_run: false,
        cmd_list: true,
        cmd_push: false,
        flag_config: String::from(config_path),
      };
      let actual = CliOptions::from(&args);
      let expected = CliOptions {
        dry_run: false,
        observed_folders: vec![PathBuf::from("projects"), PathBuf::from("code")]
      };
      assert_eq!(actual, Ok(expected));
      clean_sandbox(config_path);
    }

    #[test]
    fn test_cli_options_absolute_path_with_absolute_path() {
      let input : String = "/some/path".into();
      assert_eq!(CliOptions::absolute_path(&input), PathBuf::from("/some/path"));
    }

    #[test]
    fn test_cli_options_absolute_path_with_relative_path() {
      let input : String = "some/path".into();
      let mut expected : PathBuf = current_dir().unwrap();
      expected.push(PathBuf::from("some/path"));
      assert_eq!(CliOptions::absolute_path(&input), expected);
    }

    #[test]
    fn test_cli_options_absolute_path_with_home_path() {
      let input : String = "~/some/path".into();
      let mut expected : PathBuf = home_dir().unwrap();
      expected.push(PathBuf::from("some/path"));
      assert_eq!(CliOptions::absolute_path(&input), expected);
    }

    impl Error {
      fn from(message : &str) -> Error {
        Error { message : String::from(message) }
      }
    }

    fn write_json_to(json : &str, config_path : &str){
      clean_sandbox(config_path);
      create_sandbox(config_path);
      let mut file : File = File::create(config_path).unwrap();
      file.write_all(json.as_bytes()).unwrap();
    }

    fn clean_sandbox(config_path : &str) {
      let _ = match Path::new(config_path).parent() {
        Some(parent) =>  fs::remove_dir_all(parent.to_str().unwrap()),
        None => fs::remove_dir_all(config_path)
      };
    }

    fn create_sandbox(config_path : &str){
      let _ = match Path::new(config_path).parent() {
        Some(parent) =>  fs::create_dir(parent.to_str().unwrap()),
        None => fs::create_dir(config_path)
      };
    }
}
