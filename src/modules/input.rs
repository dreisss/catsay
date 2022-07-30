use std::{env::args, io::stdin};

use super::utils::Utilities;

#[derive(Clone)]
pub struct Input<'a> {
  pub value_: Vec<String>,
  pub type_: &'a str,
}

impl Input<'static> {
  fn get_args() -> Vec<String> {
    let mut input: Vec<String> = args().collect();
    input.remove(0);
    return input;
  }

  fn get_stdin() -> Vec<String> {
    stdin().lines().map(|l| l.expect("⚠ get_stdin error!")).collect()
  }

  fn format_stdin_input(self) -> Vec<String> {
    Utilities::fill_lines(self.value_.into_iter().map(|l: String| l.replace('\t', " ")).collect())
  }

  fn format_args_input(self) -> Vec<String> {
    let text: String = Utilities::separe_lines(self.value_.join(" "), 40);

    return Utilities::fill_lines(
      text.lines().into_iter().map(|s: &str| String::from(s)).collect(),
    );
  }

  pub fn get() -> Self {
    let (mut value_, mut type_): (Vec<String>, &str) = (Self::get_args(), "args");

    if value_.is_empty() {
      (value_, type_) = (Self::get_stdin(), "stdin");
    }

    Self { value_, type_ }
  }

  pub fn format(self) -> Vec<String> {
    if self.type_ == "stdin" {
      return self.format_stdin_input();
    }

    self.format_args_input()
  }
}
