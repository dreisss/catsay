pub mod modules;

use modules::{
  input::{Input, get_input},
  cmds::get_commands,
  utils::format_input
};

fn main() {
  let input: Input = get_input();
  if get_commands(input.clone()) {
    return
  }

  let result: Vec<String> = format_input(input);
  print(result);
}

// Print the lines formatted.
fn print(lines: Vec<String>) {
  if lines.is_empty() {
    return
  }

  let length: usize = lines[0].len();
  println!("          _-{}-_", "-".repeat(length));
  for line in lines {
    print!("         |  {}  |\n", line);
  }
  println!("        / ¯-{}-¯", "-".repeat(length));
  println!(r#"       /
   ／l、
  （ﾟ､ ｡ ７
  |、ﾞ ~ヽ
  じし f\_, )ノ"#);
}
