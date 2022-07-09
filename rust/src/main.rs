use std::{env::{args, Args}, io::{Stdin, stdin, Read}};

fn main() {
  let lines: Vec<String> = if args().len() > 1
    { format_args(args()) } else { format_pipe(&mut stdin()) };

  print_cat(lines);
}

// -----------------------------------------------------------------------< Util functions

fn lines_max_length(lines: Vec<String>) -> usize {
  if lines.is_empty() { 0 } else {
    lines.into_iter().map( |line: String| line.len() ).max().expect("Lines has no max length!")
  }
} // If the input is empty return 0 and if doesn't return the maximum length

fn fill_lines_whitespace(lines: Vec<String>) -> Vec<String> {
  let length: usize = lines_max_length(lines.clone());
  let lines: Vec<String> = lines.into_iter().map(
    |mut line: String| -> String {
      line.push_str(" ".repeat(length - line.len()).as_str());
      return line;
    }).collect();

  lines
} // Fill lines with whitespaces to the maximum length and return them

// ----------------------------------------------------------------------< Parse functions

fn format_args(args: Args) -> Vec<String> {
  let mut text_vector: Vec<String> = args.collect();
  text_vector.remove(0);

  let mut count: usize = 0;
  let text: String = text_vector.join(" ").chars().map(
    |character: char| -> char {
      if count >= 40 && character == ' ' {
        count = 0;
        return '\n';
      }
      count += 1;
      return character;
    }).collect();

  let lines: Vec<String> = text.lines().into_iter().map( |s: &str| String::from(s) ).collect();
  fill_lines_whitespace(lines)
} // Format input received from args and return them

fn format_pipe(pipe: &mut Stdin) -> Vec<String> {
  let mut text: String = String::new();
  let _result = pipe.read_to_string(&mut text);

  let lines: Vec<String> = text.lines().into_iter().map(
    |line: &str| line.replace('\t', " ")
    ).collect();

  fill_lines_whitespace(lines)
} // Format input received from pipe and return them

// -----------------------------------------------------------------------< Main functions

fn print_cat(lines: Vec<String>) {
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
} // Print text formatted and the cat
