pub fn reply(message: &str) -> &str {
    //todo!("have Bob reply to the incoming message: {message}")
    let is_question = message.trim_end().ends_with('?');
    let is_empty = message.trim().is_empty();
    let is_yelling = message.chars().any(char::is_alphabetic) && message == message.to_uppercase();
    match (is_yelling, is_question, is_empty) {
        (true, true, _) => "Calm down, I know what I'm doing!",
        (true, _, _)    => "Whoa, chill out!",
        (_, true, _)    => "Sure.",
        (_, _, true)    => "Fine. Be that way!",
        _               => "Whatever.",
    }
}