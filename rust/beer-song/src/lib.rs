fn state(n: u32, cap: bool) -> String {
    match n {
        0 => {
            if cap {
                return "No more bottles of beer".to_string();
            } else {
                return "no more bottles of beer".to_string();
            }
        }
        1 => return "1 bottle of beer".to_string(),
        _ => return format!("{} bottles of beer", n),
    }
}

fn action(n: u32) -> String {
    match n {
        0 => return "Go to the store and buy some more".to_string(),
        1 => return "Take it down and pass it around".to_string(),
        _ => return "Take one down and pass it around".to_string(),
    }
}

pub fn verse(n: u32) -> String {
    let after = match n {
        0 => state(99, false),
        _ => state(n - 1, false),
    };
    let action = action(n);

    return format!(
        "{} on the wall, {}.\n{action}, {after} on the wall.\n",
        state(n, true),
        state(n, false),
        action = action,
        after = after
    );
}

pub fn sing(start: u32, end: u32) -> String {
    let mut out_string = String::new();

    for i in (end..=start).rev() {
        out_string.push_str(&verse(i));
        if i != end {
            out_string.push_str("\n");
        }
    }
    out_string
}
