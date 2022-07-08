fn split_lines(text: String) -> (Vec<String>, usize) {
  // auxiliar variables
  let mut line: String = String::from("");
  let mut lines: Vec<String> = vec![];
  let mut character_count: usize = 0;

  // output variables
  let mut max_size: usize = 0;
  let mut lines_output: Vec<String> = vec![];

  // split text in lines and generate the max length
  for (i, character) in text.chars().enumerate() {
    if ((character_count >= 40) && (character == ' ')) || i == text.len() - 1 {
      if character != ' ' { line.push(character) }
      lines.push(line.clone());
      line.clear();

      max_size = if character_count + 1 > max_size { character_count + 1 } else { max_size };
      character_count = 0;
      continue;
    }

    line.push(character);
    character_count += 1;
  }

  // grant lines has max_size length
  for mut line in lines {
    while line.len() < max_size {
      line.push(' ');
    }
    lines_output.push(line);
  }

  return (lines_output, max_size);
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
  // Receive command text
  let mut input_array: Vec<String> = std::env::args().collect();
  input_array.remove(0);

  // split and format text in lines
  let input_text: String = input_array.join(" ");
  let lines: (Vec<String>, usize) = split_lines(input_text);

  // Print output
  print_cat(lines.0, lines.1);
}
