use std::{env::args, io::stdin};

#[derive(Clone)]
pub struct Input {
  pub _input: Vec<String>,
  pub _type: isize,
}

/// Read cli and pipe and return one of them (cli has priority).
pub fn get_input() -> Input {
  let mut input: Vec<String> = args().collect();
  input.remove(0);

  if !input.is_empty() { Input {_input: input, _type: 1} } else {
    Input {
      _input: stdin().lines().into_iter()
        .map( |line| line.expect("Error receiving input!")).collect(),
      _type: 2
    }
  }
}
