use std::{
    fs,
    error::Error,
};

const INPUTS_PATH: &str = "/src/inputs/";

pub struct Context{
    // string contents of the file to work with
    pub input: String,
    _path: String,
}

impl Context{
    pub fn from (day: &str, is_test: bool) -> Result<Context, Box<dyn Error>> {
        let path: String = std::env::current_dir().unwrap().to_str().unwrap().to_owned() + INPUTS_PATH + "day-" + day + if is_test {"-test.txt"} else {".txt"};
        let input = fs::read_to_string(&path)?;
        println!("Creating Context with file: {}", path);
        Ok(Context{input, _path: path})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_file() {
        let mut runner: Option<Context> = None;
        match Context::from("0", true){
            Ok(val) => runner = Some(val),
            Err(msg) => {eprintln!("Failed with error: {}", msg); ()},
        }
        assert!(runner.is_none());
    }

    #[test]
    fn valid_file() {
        let path = std::env::current_dir().unwrap().to_str().unwrap().to_owned() + "/src/inputs/day-6-test.txt";
        let input = fs::read_to_string(&path).unwrap();
        let mut runner: Option<Context> = None;
        match Context::from("6", true){
            Ok(val) => runner = Some(val),
            Err(msg) => {eprintln!("Failed with error: {}", msg); ()},
        }
        assert!(!runner.is_none());
        assert_eq!(runner.as_ref().unwrap()._path, path);
        assert_eq!(runner.as_ref().unwrap().input, input);
    }
}