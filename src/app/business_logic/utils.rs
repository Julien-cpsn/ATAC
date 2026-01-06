pub fn to_train_case(text: &String) -> String {
    if text.trim().is_empty() {
        return text.to_owned();
    }

    let mut chars = text.chars();
    let mut new_chars = Vec::new();

    let first_char = chars.next().unwrap();
    new_chars.push(first_char.to_ascii_uppercase());

    while let Some(c) = chars.next() {
        if c == '-' {
            if let Some(next) = chars.next() {
                new_chars.push(c);
                new_chars.push(next.to_ascii_uppercase());
            }
        }
        else {
            new_chars.push(c);
        }
    }

    return new_chars.iter().collect();
}