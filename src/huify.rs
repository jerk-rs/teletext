const VOWELS: [char; 10] = ['а', 'е', 'ё', 'и', 'о', 'у', 'э', 'ы', 'ю', 'я'];

pub fn huify_word(s: &str) -> Option<String> {
    let mut chars = s.chars().skip_while(|c| !VOWELS.contains(c));
    let first = match chars.next() {
        Some(c) => c,
        None => return None,
    };
    let mut result = format!(
        "ху{}",
        match first {
            'о' => 'ё',
            'а' => 'я',
            'у' => 'ю',
            'ы' => 'и',
            c => c,
        }
    );
    result.extend(chars);
    Some(result)
}

pub fn huify_sentence(s: &str) -> String {
    let mut result = String::with_capacity(s.len() * 2);
    for (idx, word) in s.split(" ").enumerate() {
        if idx != 0 {
            result.push(' ');
        }
        if let Some(huified) = huify_word(word) {
            result += &huified;
        } else {
            result += word;
        }
    }
    result
}
