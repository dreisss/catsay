pub mod modules;

use modules::{cmds::Command, input::Input};

fn main() {
  let input: Input = Input::get();
  let command: Command = Command::get(input.clone());
  command.clone().run();

  if command.clone()._break_execution {
    return;
  }

  let result: Vec<String> = input.format();
  print(result);
}

// Print the lines formatted.
fn print(lines: Vec<String>) {
  if lines.is_empty() {
    return;
  }

  let length: usize = lines[0].len();
  println!("          _-{}-_", "-".repeat(length));
  for line in lines {
    print!("         |  {}  |\n", line);
  }
  println!("        / ¯-{}-¯", "-".repeat(length));
  println!(
    r#"       /
   ／l、
  （ﾟ､ ｡ ７
  |、ﾞ ~ヽ
  じし f\_, )ノ"#
  );
}
