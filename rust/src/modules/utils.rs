pub struct Utilities;

impl Utilities {
  fn list_lengths(list: Vec<String>) -> Vec<usize> {
    list.into_iter().map(|s: String| s.len()).collect()
  }

  fn max_length(list: Vec<String>) -> usize {
    Self::list_lengths(list).into_iter().max().expect("⚠ max_length error!")
  }

  fn fill_line(mut line: String, length: usize) -> String {
    line.push_str(" ".repeat(length - line.len()).as_str());
    return line;
  }

  pub fn fill_lines(lines: Vec<String>) -> Vec<String> {
    let max_length: usize = Self::max_length(lines.clone());
    return lines.into_iter().map(|l: String| Self::fill_line(l, max_length)).collect();
  }

  pub fn separe_lines(text: String, limit_length: usize) -> String {
    let (mut count_, mut char_): (usize, char) = (0, '\n');

    text
      .chars()
      .map(|character: char| -> char {
        char_ = character;

        if count_ >= limit_length && character == ' ' {
          (count_, char_) = (0, '\n');
        }

        count_ += 1;
        return char_;
      })
      .collect()
  }
}
