import gleam/string

pub fn message(log_line: String) -> String {
  case log_line {
    "[INFO]:" <> msg -> string.trim(msg)
    "[WARNING]:" <> msg -> string.trim(msg)
    "[ERROR]:" <> msg -> string.trim(msg)
    _ -> "unknown format"
  }
}

pub fn log_level(log_line: String) -> String {
  case log_line {
    "[INFO]:" <> msg -> "info"
    "[WARNING]:" <> msg -> "warning"
    "[ERROR]:" <> msg -> "error"
    _ -> "unknown format"
  }
}

pub fn reformat(log_line: String) -> String {
  message(log_line) <> " (" <> log_level(log_line) <> ")"
}


