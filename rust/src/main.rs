use std::{env::{args, Args}, io::{Stdin, stdin, Read}};

fn split_lines_args(args: &mut Args) -> (Vec<String>, usize) {
  let mut text_vector: Vec<String> = args.collect();
  text_vector.remove(0);
  let text: String = text_vector.join(" ");

  // auxiliar variables
  let mut temp_line: String = String::new();
  let mut temp_lines: Vec<String> = Vec::new();
  let mut temp_count: usize = 0;

  // output variables
  let mut max_size: usize = 0;
  let mut lines: Vec<String> = Vec::new();

  // split text in lines and generate the max length
  for (i, character) in text.chars().enumerate() {
    if ((temp_count >= 40) && (character == ' ')) || i == text.len() - 1 {
      if character != ' ' { temp_line.push(character) }
      temp_lines.push(temp_line.clone());
      temp_line.clear();

      max_size = if temp_count + 1 > max_size { temp_count + 1 } else { max_size };
      temp_count = 0;
      continue;
    }

    temp_line.push(character);
    temp_count += 1;
  }

  // grant lines has max_size length
  for mut line in temp_lines {
    while line.len() < max_size {
      line.push(' ');
    }
    lines.push(line);
  }

  return (lines, max_size);
}

fn split_lines_pipe(pipe: &mut Stdin) -> (Vec<String>, usize) {
  let mut text: String = String::new();
  let _result = pipe.read_to_string(&mut text);

  let mut temp_lines: Vec<String> = Vec::new();

  let mut lines: Vec<String> = Vec::new();
  let mut max_size: usize = 0;

  for line in text.lines() {
    temp_lines.push(String::from(line.replace('\t', " ")));
    max_size = if line.len() > max_size { line.len() } else { max_size };
  }

  for mut line in temp_lines {
    while line.len() < max_size {
      line.push(' ');
    }
    lines.push(line);
  }

  return (lines, max_size);
}

fn print_cat(lines: Vec<String>, max_size: usize) {
  println!("          _-{}-_", "-".repeat(max_size));
  for line in lines {
    print!("         |  {}  |\n", line);
  }
  println!("        / ¯-{}-¯", "-".repeat(max_size));
  println!("       /     ");
  println!(" ／l、        ");
  println!("（ﾟ､ ｡ ７     ");
  println!("|、ﾞ ~ヽ      ");
  println!("じし f\\_, )ノ");
}

fn main() {
  // Define util variable
  let mut lines: (Vec<String>, usize);

  // Receive command text when args
  let mut input_args: Args = args();
  lines = split_lines_args(&mut input_args);

  // Receive command form pipe and use if there's no other input
  let mut input_pipe: Stdin = stdin();
  if lines.0.is_empty() {
    lines = split_lines_pipe(&mut input_pipe);
  }

  // Print output
  print_cat(lines.0, lines.1);
}
