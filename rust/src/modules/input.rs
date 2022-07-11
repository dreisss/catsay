use std::{env::args, io::stdin};

pub fn get_input() -> (Vec<String>, &'static str) {
  let mut input: Vec<String> = args().collect();
  input.remove(0);

  if !input.is_empty() { (input, "type1") } else {
    (stdin().lines().into_iter().map(
      |line| line.expect("Error receiving input!")
    ).collect(), "type2")
  }
}
