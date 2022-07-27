use super::input::Input;

#[derive(Clone)]
pub struct Command<'a> {
  pub _name: &'a str,
  pub _break_execution: bool,
  _function: fn(String),
  _arg: String,
}

struct CommandDoc<'a> {
  _name: &'a str,
  _extended: &'a str,
  _compact: &'a str,
  _break_execution: bool,
  _function: fn(String),
  _has_arg: bool,
}

impl Command<'static> {
  // Receiving command from given input
  pub fn get(input: Input) -> Command {
    if input.type_ == "default" && input.value_[0].starts_with("-") {
      for command in DOC_COMMANDS.iter() {
        if command._compact == input.value_[0] || command._extended == input.value_[0] {
          return Command {
            _name: command._name,
            _function: command._function,
            _arg: if command._has_arg {
              input.value_[1].clone()
            } else {
              NO_COMMAND._arg
            },
            _break_execution: command._break_execution,
          };
        }
      }
    }
    NO_COMMAND
  }

  // Running command
  pub fn run(self) {
    (self._function)(self._arg);
  }
}

const NO_COMMAND: Command = Command {
  _name: "",
  _function: |_string: String| {},
  _arg: String::new(),
  _break_execution: false,
};

const DOC_COMMANDS: [CommandDoc; 2] = [
  // Version command
  CommandDoc {
    _name: "version",
    _extended: "--version",
    _compact: "-v",
    _break_execution: true,
    _function: |_arg: String| println!("catsay v{}", env!("CARGO_PKG_VERSION")),
    _has_arg: false,
  },
  // Help command
  CommandDoc {
    _name: "help",
    _extended: "--help",
    _compact: "-h",
    _break_execution: true,
    _function: |_arg: String| {
      println!(
        r#"
  A simple cowsay but with a cat!

  + Usability:
    Use with default cli input:
      catsay Hello, I am a cat
      catsay "Hello, I'm a cat'

    Use with shell pipe:
      fortune | catsay
      ls | catsay

  + Commands:
    -h | --help: Help command to the program
    -v | --version: Displays the program version
      "#
      )
    },
    _has_arg: false,
  },
];
