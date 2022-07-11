use super::input::Input;

/// Get commands in the input.
pub fn get_commands(input: Input) -> bool {
    for word in input._input {
      if word.starts_with("-") {
        return match word.as_str() {
          "--version" | "-v" => __version__(),
          &_ => false
        }
      }
    }
  false
}

/// Print the package version.
fn __version__() -> bool {
  println!("catsay v{}", env!("CARGO_PKG_VERSION"));
  true
}
