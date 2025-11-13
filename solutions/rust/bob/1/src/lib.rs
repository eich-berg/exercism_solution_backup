pub fn reply(message: &str) -> &str {
    // todo!("have Bob reply to the incoming message: {message}")
    if message.trim().is_empty() { return "Fine. Be that way!"; }
    let is_question = message.trim().ends_with('?');
    let is_uppercase = message.chars().any(|ch| ch.is_alphabetic()) && message == message.to_uppercase();
    match (is_uppercase, is_question) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, _) => "Whoa, chill out!",
        (_, true) => "Sure.",
        _ => "Whatever.",
    }
}