pub fn reply(msg: &str) -> &str {
    let msg = msg.trim();
    let is_all_upper = |msg: &str| msg.chars()
                                      .filter(|c| c.is_alphabetic())
                                      .all(|c| c.is_uppercase()) ;
    let is_question = |msg: &str| msg.chars().last() == Some('?');

    match msg {
        _ if msg.is_empty() => "Fine. Be that way!",
        _ if is_all_upper(msg) => "Whoa, chill out!",
        _ if is_question(msg) => "Sure.",
        _ => "Whatever."
    }
}
