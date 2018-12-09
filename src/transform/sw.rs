use super::{
    utils::{collect_uppercase_chars, validate_len},
    TransformResult,
};

pub fn transform(orig: &str) -> TransformResult<String> {
    validate_len(3, 100, orig.len())?;
    let chars = collect_uppercase_chars(&orig);
    let len = chars.len();
    let mut buf = String::new(); // TODO: calc capacity

    // top lines
    for (a, b) in (0..len).zip((1..len).rev()) {
        buf.push(chars[a]);
        buf.extend(vec![' '; (len - 2) * 2 + 1]);
        if a == 0 {
            for x in chars.iter().rev() {
                buf.push(*x);
                buf.push(' ');
            }
        } else {
            buf.push(chars[b]);
        }
        buf.push('\n');
    }

    // middle line
    for &c in chars.iter().skip(1).rev() {
        buf.push(c);
        buf.push(' ');
    }
    for (i, &c) in chars.iter().enumerate() {
        buf.push(c);
        if i == len - 1 {
            buf.push('\n')
        } else {
            buf.push(' ')
        }
    }

    // bottom lines
    for (a, b) in (1..len).zip((0..len - 1).rev()) {
        if b == 0 {
            for x in chars.iter() {
                buf.push(*x);
                buf.push(' ');
            }
            buf.extend(vec![' '; (len - 2) * 2]);
        } else {
            buf.extend(vec![' '; (len - 1) * 2]);
            buf.push(chars[a]);
            buf.extend(vec![' '; (len - 2) * 2 + 1]);
        }
        buf.push(chars[b]);
        buf.push('\n');
    }

    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok() {
        let transformed = transform("rurust").unwrap();
        let mut lines = transformed.lines();
        assert_eq!(lines.next(), Some("R         T S U R U R "));
        assert_eq!(lines.next(), Some("U         S"));
        assert_eq!(lines.next(), Some("R         U"));
        assert_eq!(lines.next(), Some("U         R"));
        assert_eq!(lines.next(), Some("S         U"));
        assert_eq!(lines.next(), Some("T S U R U R U R U S T"));
        assert_eq!(lines.next(), Some("          U         S"));
        assert_eq!(lines.next(), Some("          R         U"));
        assert_eq!(lines.next(), Some("          U         R"));
        assert_eq!(lines.next(), Some("          S         U"));
        assert_eq!(lines.next(), Some("R U R U S T         R"));
    }

    #[test]
    fn err() {
        let expected = String::from("Text must contain from 3 up to 100 characters");

        let err = transform("").unwrap_err();
        assert_eq!(err.to_string(), expected);

        let err = transform("aa").unwrap_err();
        assert_eq!(err.to_string(), expected);

        let err = transform(&"a".repeat(101)).unwrap_err();
        assert_eq!(err.to_string(), expected);

        assert_eq!(transform(&"a".repeat(3)).is_ok(), true);
        assert_eq!(transform(&"a".repeat(100)).is_ok(), true);
    }
}
