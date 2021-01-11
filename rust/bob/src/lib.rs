pub fn reply(message: &str) -> &str {
    let no_whitespace = message
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();

    if no_whitespace == "" {
        return "Fine. Be that way!";
    }

    let letter_iter = no_whitespace
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<String>();

    let all_caps: bool;

    if letter_iter.len() == 0 {
        all_caps = false;
    } else {
        all_caps = letter_iter.chars().all(|c| c.is_uppercase());
    }

    let last = no_whitespace.chars().last().unwrap();

    match (all_caps, last) {
        (false, '?') => return "Sure.",
        (true, '?') => return "Calm down, I know what I'm doing!",
        (true, _) => return "Whoa, chill out!",
        _ => return "Whatever.",
    }
}
