const VOWELS: [char; 10] = ['а', 'е', 'ё', 'и', 'о', 'у', 'э', 'ы', 'ю', 'я'];

fn should_huify(s: &str) -> bool {
    let mut chars = s.chars().peekable();
    match chars.next() {
        Some('х') => match chars.next() {
            Some('у') => chars.peek().map(|c| !VOWELS.contains(&c)).unwrap_or(true),
            Some(_) => true,
            None => false,
        },
        Some(_) => chars.peek().is_some(),
        None => false,
    }
}

pub fn huify_word(s: &str) -> Option<String> {
    println!("{:?} {:?}", s, s.len());
    if s.len() == 1 {
        return None;
    }
    if !should_huify(s) {
        return None;
    }
    let mut tail = s.chars().skip_while(|c| !VOWELS.contains(c));
    let first = match tail.next() {
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
            'э' => 'е',
            c => c,
        }
    );
    result.extend(tail);
    Some(result)
}

pub fn huify_sentence(s: &str) -> String {
    let mut result = String::with_capacity(s.len() * 2);
    for (idx, word) in s.to_lowercase().split_whitespace().enumerate() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_huify_sentence() {
        for (input, expected) in vec![
            ("Значимость этих проблем настолько очевидна", "хуячимость хуетих хуёблем хуястолько хуёчевидна"),
            ("Андрей", "хуяндрей"),
            ("imported and not used\n\nдевиз моей жизни", "imported and not used хуевиз хуёей хуизни"),
            ("ХУЁВОЕ НАСТРОЕНИЕ", "хуёвое хуястроение"),
            ("ЁБАНАЯ ХУНТА", "хуёбаная хуюнта"),
            ("аутизм и деградация", "хуяутизм и хуеградация"),
            ("ху", "хую"),
            ("хуякс", "хуякс")
        ] {
            assert_eq!(huify_sentence(input), expected);
        }
    }
}
