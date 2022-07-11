pub fn max_length(list: Vec<String>) -> usize {
  if list.is_empty() { 0 } else {
    list.into_iter().map(
      |line: String| line.len() ).max()
    .expect("Lines has no max length!")
  }
}

pub fn fill_all_in_list(list: Vec<String>) -> Vec<String> {
  let max_length: usize = max_length(list.clone());
  list.into_iter().map(
    |mut line: String| -> String {
      line.push_str(" ".repeat(max_length - line.len()).as_str());
      return line;
    }).collect()
}

pub fn format_input(input: Vec<String>, input_type: &str) -> Vec<String> {
  if input_type == "type1" {
    let mut count: usize = 0;
    let text: String = input.join(" ").chars().map(
      |character: char| -> char {
        if count >= 40 && character == ' ' {
          count = 0; return '\n';
        }
        count += 1; return character;
      }).collect();

    let lines: Vec<String> = text.lines().into_iter().map( |s: &str| String::from(s) ).collect();
    fill_all_in_list(lines)
  } else {
    let lines: Vec<String> = input.into_iter().map(
      |line: String| line.replace('\t', " ")
      ).collect();

    fill_all_in_list(lines)
  }
}
