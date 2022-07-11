pub fn get_commands(input: Vec<String>, input_type: &str) -> bool {
  if input_type == "type1" {
    for word in input {
      if word.starts_with("-") {
        return match word.as_str() {
          "--version" | "-v" => __version__(),
          &_ => false
        }
      }
    }
  }
  false
}

fn __version__() -> bool {
  println!("catsay v{}", env!("CARGO_PKG_VERSION"));
  true
}
