extern crate regex;

use regex::Regex;

pub struct Transformer {
    cmd_pattern: Regex,
}

impl Transformer {
    pub fn new() -> Transformer {
        Transformer {
            cmd_pattern: Regex::new(r"([A-z_]+)(@[A-z_]+)?(.*)").unwrap(),
        }
    }

    pub fn transform(&self, data: &str) -> Option<String> {
        if let Some(caps) = self.cmd_pattern.captures(data) {
            match (caps.get(1), caps.get(3)) {
                (Some(cmd), Some(text)) => {
                    let text = text.as_str().trim();
                    let text_size = text.len();
                    if text_size < 3 || text_size > 500 {
                        return None;
                    }
                    Some(match cmd.as_str() {
                        "square" => to_square(text),
                        "star" => to_star(text),
                        "qstar" => to_qstar(text),
                        _ => return None,
                    })
                }
                _ => None,
            }
        } else {
            None
        }
    }
}

fn to_square(s: &str) -> String {
    let s: Vec<char> = s.chars().flat_map(|c| c.to_uppercase()).collect();
    let len = s.len();
    let side = len * 2 - 1;
    let area = side * side;
    let mut buf = String::with_capacity(area * 2 - 1);
    let mut row_idx;
    let mut col_idx;
    for row in 0..side {
        row_idx = if row >= len { side - row - 1 } else { row };
        for col in 0..side {
            col_idx = if col >= len { side - col - 1 } else { col };
            buf.push(s[len - 1 - if row_idx <= col_idx { row_idx } else { col_idx }]);
            if col != side - 1 {
                buf.push(' ');
            }
        }
        if row != side - 1 {
            buf.push('\n');
        }
    }
    buf
}

fn to_star(s: &str) -> String {
    let chars = s.chars().flat_map(|c| c.to_uppercase()).collect::<Vec<_>>();
    let sqr = |x| x * x;
    let len = chars.len();
    let mut output = String::with_capacity(sqr(len * 2));

    // top lines
    for (i, &c) in chars.iter().skip(1).enumerate().rev() {
        for _ in 0..(len - i - 2) * 2 {
            output.push(' ');
        }
        output.push(c);
        output.push(' ');
        for _ in 0..i * 2 {
            output.push(' ');
        }
        output.push(c);
        output.push(' ');
        for _ in 0..i * 2 {
            output.push(' ');
        }
        output.push(c);
        output.push('\n');
    }

    // middle line
    for &c in chars.iter().skip(1).rev() {
        output.push(c);
        output.push(' ');
    }
    for (i, &c) in chars.iter().enumerate() {
        output.push(c);
        if i == len - 1 {
            output.push('\n')
        } else {
            output.push(' ')
        }
    }

    // bottom lines
    for (i, &c) in chars.iter().skip(1).enumerate() {
        for _ in 0..(len - i - 2) * 2 {
            output.push(' ');
        }
        output.push(c);
        output.push(' ');
        for _ in 0..i * 2 {
            output.push(' ');
        }
        output.push(c);
        output.push(' ');
        for _ in 0..i * 2 {
            output.push(' ');
        }
        output.push(c);
        output.push('\n');
    }

    output
}

fn to_qstar(input: &str) -> String {
    let chars = input
        .chars()
        .flat_map(|c| c.to_uppercase())
        .collect::<Vec<_>>();
    let sqr = |x| x * x;
    let len = chars.len();
    let mut output = String::with_capacity(sqr(len * 2));

    // top line
    for (i, &c) in chars.iter().enumerate() {
        output.push(c);
        if i == len - 1 {
            output.push('\n')
        } else {
            output.push(' ')
        }
    }

    // bottom lines
    for (i, &c) in chars.iter().skip(1).enumerate() {
        output.push(c);
        output.push(' ');
        for _ in 0..i * 2 {
            output.push(' ');
        }
        output.push(c);
        output.push('\n');
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_text_square(text: &str) {
        let mut lines = text.lines();
        assert_eq!(lines.next(), Some("T T T T T T T"));
        assert_eq!(lines.next(), Some("T X X X X X T"));
        assert_eq!(lines.next(), Some("T X E E E X T"));
        assert_eq!(lines.next(), Some("T X E T E X T"));
        assert_eq!(lines.next(), Some("T X E E E X T"));
        assert_eq!(lines.next(), Some("T X X X X X T"));
        assert_eq!(lines.next(), Some("T T T T T T T"));
    }

    fn assert_text_star(text: &str) {
        let mut lines = text.lines();
        assert_eq!(lines.next(), Some("T     T     T"));
        assert_eq!(lines.next(), Some("  X   X   X"));
        assert_eq!(lines.next(), Some("    E E E"));
        assert_eq!(lines.next(), Some("T X E T E X T"));
        assert_eq!(lines.next(), Some("    E E E"));
        assert_eq!(lines.next(), Some("  X   X   X"));
        assert_eq!(lines.next(), Some("T     T     T"));
    }

    fn assert_text_qstar(text: &str) {
        let mut lines = text.lines();
        assert_eq!(lines.next(), Some("T E X T"));
        assert_eq!(lines.next(), Some("E E"));
        assert_eq!(lines.next(), Some("X   X"));
        assert_eq!(lines.next(), Some("T     T"));
    }

    #[test]
    fn it_works() {
        let tf = Transformer::new();

        for i in vec!["/square@bot text", "/square text", "/square@s_oMe_bot text"] {
            let data = tf.transform(i).unwrap();
            assert_text_square(&data);
        }

        for i in vec!["/star@bot text", "/star text", "/star@s_oMe_bot text"] {
            let data = tf.transform(i).unwrap();
            assert_text_star(&data);
        }

        for i in vec!["/qstar@bot text", "/qstar text", "/qstar@s_oMe_bot text"] {
            let data = tf.transform(i).unwrap();
            assert_text_qstar(&data);
        }
    }

    #[test]
    fn it_not_works() {
        let tf = Transformer::new();
        for i in vec![
            "",
            "/square",
            "/star ",
            "/qstar x",
            "/square xx",
            &format!("/star {}", String::from_utf8(vec![b'X'; 501]).unwrap()),
        ] {
            assert_eq!(tf.transform(&i).is_none(), true);
        }
    }
}
