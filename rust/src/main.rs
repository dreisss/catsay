pub mod modules;
use modules::{
  input::get_input,
  cmds::get_commands,
  utils::format_input
};

fn main() {
  let (input, input_type): (Vec<String>, &str) = get_input();
  if get_commands(input.clone(), input_type) {
    return
  }

  let result: Vec<String> = format_input(input, input_type);
  print(result);
}

fn print(lines: Vec<String>) {
  if lines.is_empty() {
    return
  }

  let length: usize = [0].len();
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
} // Print text formatted and the cat
