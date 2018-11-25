use super::{collect_chars, Bounds, TransformError, TransformResult};
const SW_BOUNDS: Bounds = (3, 100);

pub fn to_sw(orig: &str) -> TransformResult<String> {
    let len = orig.len();
    if len < SW_BOUNDS.0 || len > SW_BOUNDS.1 {
        return Err(TransformError::invalid_length(SW_BOUNDS.0, SW_BOUNDS.1));
    }

    let chars = collect_chars(&orig);
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
    fn sw() {
        let transformed = to_sw("rurust").unwrap();
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
}
