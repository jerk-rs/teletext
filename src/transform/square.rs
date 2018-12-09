use super::{collect_chars, Bounds, Error, Result};

const SQUARE_BOUNDS: Bounds = (2, 100);

pub fn to_square(origin: &str) -> Result<String> {
    let len = origin.len();
    if len < SQUARE_BOUNDS.0 || len > SQUARE_BOUNDS.1 {
        return Err(Error::InvalidLength {
            min: SQUARE_BOUNDS.0,
            max: SQUARE_BOUNDS.1,
        });
    }

    let chars = collect_chars(&origin);
    let len = chars.len();
    let side = len * 2 - 1;
    let area = side * side;
    let mut buf = String::with_capacity(area * 2 - 1);
    let mut row_idx;
    let mut col_idx;
    for row in 0..side {
        row_idx = if row >= len { side - row - 1 } else { row };
        for col in 0..side {
            col_idx = if col >= len { side - col - 1 } else { col };
            buf.push(chars[len - 1 - if row_idx <= col_idx { row_idx } else { col_idx }]);
            if col != side - 1 {
                buf.push(' ');
            }
        }
        if row != side - 1 {
            buf.push('\n');
        }
    }
    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square() {
        let transformed = to_square("text").unwrap();
        let mut lines = transformed.lines();
        assert_eq!(lines.next(), Some("T T T T T T T"));
        assert_eq!(lines.next(), Some("T X X X X X T"));
        assert_eq!(lines.next(), Some("T X E E E X T"));
        assert_eq!(lines.next(), Some("T X E T E X T"));
        assert_eq!(lines.next(), Some("T X E E E X T"));
        assert_eq!(lines.next(), Some("T X X X X X T"));
        assert_eq!(lines.next(), Some("T T T T T T T"));
    }
}
