pub fn reply(guess: Int) -> String {
  case guess {
    42 -> "Correct"
    guess if guess < 41 -> "Too low"
    guess if guess > 43 -> "Too high"
    _ -> "So close"
  }
}