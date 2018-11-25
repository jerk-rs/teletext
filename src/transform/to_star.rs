use super::{collect_chars, Bounds, Error, Result};
const STAR_BOUNDS: Bounds = (3, 100);

pub fn to_star(orig: &str) -> Result<String> {
    let len = orig.len();
    if len < STAR_BOUNDS.0 || len > STAR_BOUNDS.1 {
        return Err(Error::InvalidLength {
            min: STAR_BOUNDS.0,
            max: STAR_BOUNDS.1,
        });
    }

    let chars = collect_chars(&orig);
    let len = chars.len();
    let mut buf = String::new(); // TODO: calc capacity

    // top lines
    for (i, &c) in chars.iter().skip(1).enumerate().rev() {
        for _ in 0..(len - i - 2) * 2 {
            buf.push(' ');
        }
        buf.push(c);
        buf.push(' ');
        for _ in 0..i * 2 {
            buf.push(' ');
        }
        buf.push(c);
        buf.push(' ');
        for _ in 0..i * 2 {
            buf.push(' ');
        }
        buf.push(c);
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
    for (i, &c) in chars.iter().skip(1).enumerate() {
        for _ in 0..(len - i - 2) * 2 {
            buf.push(' ');
        }
        buf.push(c);
        buf.push(' ');
        for _ in 0..i * 2 {
            buf.push(' ');
        }
        buf.push(c);
        buf.push(' ');
        for _ in 0..i * 2 {
            buf.push(' ');
        }
        buf.push(c);
        buf.push('\n');
    }
    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star() {
        let transformed = to_star("text").unwrap();
        let mut lines = transformed.lines();
        assert_eq!(lines.next(), Some("T     T     T"));
        assert_eq!(lines.next(), Some("  X   X   X"));
        assert_eq!(lines.next(), Some("    E E E"));
        assert_eq!(lines.next(), Some("T X E T E X T"));
        assert_eq!(lines.next(), Some("    E E E"));
        assert_eq!(lines.next(), Some("  X   X   X"));
        assert_eq!(lines.next(), Some("T     T     T"));
    }
}
