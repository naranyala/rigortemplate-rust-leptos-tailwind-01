use std::fmt::Write;

const KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "else",
    "enum", "extern", "false", "fn", "for", "if", "impl", "in", "let",
    "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "self",
    "static", "struct", "super", "trait", "true", "type", "unsafe", "use",
    "where", "while",
];

const TYPES: &[&str] = &[
    "bool", "char", "f32", "f64", "i8", "i16", "i32", "i64", "i128",
    "isize", "u8", "u16", "u32", "u64", "u128", "usize", "str",
    "String", "Vec", "Option", "Result", "Box", "Rc", "Arc", "Cell",
    "RefCell", "HashMap", "HashSet", "AnyView", "RwSignal", "Memo",
    "Children", "Fn", "FnMut", "FnOnce",
];

fn is_keyword(word: &str) -> bool {
    KEYWORDS.contains(&word)
}

fn is_type(word: &str) -> bool {
    TYPES.contains(&word)
}

fn highlight_line(line: &str) -> String {
    let mut out = String::new();
    let mut chars = line.char_indices().peekable();

    while let Some((i, c)) = chars.next() {
        if c == '/' && chars.peek().map(|(_, c)| *c) == Some('/') {
            let _ = write!(out, "<span class=\"hl-comment\">{}</span>", &line[i..]);
            break;
        }

        if c == '/' && chars.peek().map(|(_, c)| *c) == Some('*') {
            let mut depth = 1;
            let mut j = i + 2;
            while let Some((k, ch)) = chars.next() {
                if ch == '/' && chars.peek().map(|(_, c)| *c) == Some('*') {
                    depth += 1;
                    chars.next();
                    j = k + 2;
                } else if ch == '*' && chars.peek().map(|(_, c)| *c) == Some('/') {
                    depth -= 1;
                    chars.next();
                    j = k + 2;
                    if depth == 0 {
                        break;
                    }
                } else {
                    j = k + ch.len_utf8();
                }
            }
            let _ = write!(out, "<span class=\"hl-comment\">{}</span>", &line[i..j]);
            break;
        }

        if c == '"' {
            let mut s = String::new();
            s.push('"');
            while let Some((_, ch)) = chars.next() {
                s.push(ch);
                if ch == '\\' {
                    if let Some((_, esc)) = chars.next() {
                        s.push(esc);
                    }
                } else if ch == '"' {
                    break;
                }
            }
            let _ = write!(out, "<span class=\"hl-string\">{}</span>", html_escape(&s));
            continue;
        }

        if c == 'r' && chars.peek().map(|(_, c)| *c) == Some('#') {
            let mut s = String::new();
            s.push('r');
            let mut _hashes = 0;
            while let Some((_, ch)) = chars.next() {
                s.push(ch);
                if ch == '#' {
                    _hashes += 1;
                } else if ch == '"' {
                    s.push('"');
                    break;
                } else {
                    break;
                }
            }
            while let Some((_, ch)) = chars.next() {
                s.push(ch);
                if ch == '"' {
                    if let Some((_, ch2)) = chars.next() {
                        s.push(ch2);
                        if ch2 == '#' {
                            break;
                        }
                    }
                }
            }
            let _ = write!(out, "<span class=\"hl-string\">{}</span>", html_escape(&s));
            continue;
        }

        if c == '\'' && chars.peek().map(|(_, c)| *c != '\'') == Some(true) {
            let mut s = String::new();
            s.push('\'');
            if let Some((_, ch)) = chars.next() {
                s.push(ch);
                if ch == '\\' {
                    if let Some((_, esc)) = chars.next() {
                        s.push(esc);
                    }
                }
                if let Some((_, ch2)) = chars.next() {
                    if ch2 == '\'' {
                        s.push('\'');
                    } else {
                        out.push_str(&html_escape(&s));
                        out.push(ch2);
                        continue;
                    }
                }
            }
            let _ = write!(out, "<span class=\"hl-string\">{}</span>", html_escape(&s));
            continue;
        }

        if c == '\'' && chars.peek().map(|(_, c)| *c == '\'') != Some(true) {
            let mut lifetime = String::new();
            lifetime.push('\'');
            while let Some((_, ch)) = chars.peek() {
                if ch.is_alphanumeric() || *ch == '_' {
                    lifetime.push(*ch);
                    chars.next();
                } else {
                    break;
                }
            }
            let _ = write!(out, "<span class=\"hl-lifetime\">{}</span>", lifetime);
            continue;
        }

        if c == '#' && chars.peek().map(|(_, c)| *c) == Some('[') {
            chars.next();
            let mut attr = String::from("#[");
            let mut depth = 1;
            while let Some((_, ch)) = chars.next() {
                attr.push(ch);
                if ch == '[' {
                    depth += 1;
                } else if ch == ']' {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
            }
            let _ = write!(out, "<span class=\"hl-attribute\">{}</span>", html_escape(&attr));
            continue;
        }

        if c.is_ascii_digit()
            || (c == '.' && chars.peek().map(|(_, c)| c.is_ascii_digit()) == Some(true))
        {
            let mut n = String::new();
            n.push(c);
            while let Some((_, ch)) = chars.peek() {
                if ch.is_ascii_digit() || *ch == '.' || *ch == '_' || ch.is_ascii_hexdigit() {
                    n.push(*ch);
                    chars.next();
                } else {
                    break;
                }
            }
            let _ = write!(out, "<span class=\"hl-number\">{}</span>", n);
            continue;
        }

        if c.is_alphabetic() || c == '_' {
            let mut word = String::new();
            word.push(c);
            while let Some((_, ch)) = chars.peek() {
                if ch.is_alphanumeric() || *ch == '_' {
                    word.push(*ch);
                    chars.next();
                } else {
                    break;
                }
            }

            if is_keyword(&word) {
                let _ = write!(out, "<span class=\"hl-keyword\">{}</span>", word);
            } else if is_type(&word) {
                let _ = write!(out, "<span class=\"hl-type\">{}</span>", word);
            } else if word.ends_with('!') && word != "!" {
                let _ = write!(out, "<span class=\"hl-macro\">{}</span>", word);
            } else if word.chars().next().map_or(false, |c| c.is_uppercase()) {
                let _ = write!(out, "<span class=\"hl-ctor\">{}</span>", word);
            } else {
                let _ = write!(out, "{}", word);
            }
            continue;
        }

        if c == '!' && chars.peek().map(|(_, ch)| ch.is_alphabetic()) == Some(true) {
            let mut macro_name = String::new();
            while let Some((_, ch)) = chars.peek() {
                if ch.is_alphanumeric() || *ch == '_' {
                    macro_name.push(*ch);
                    chars.next();
                } else {
                    break;
                }
            }
            let _ = write!(out, "<span class=\"hl-macro\">!{}</span>", macro_name);
            continue;
        }

        out.push(c);
    }

    out
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

pub fn highlight_rust(source: &str) -> String {
    let mut out = String::new();
    for line in source.lines() {
        let _ = writeln!(out, "{}", highlight_line(line));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highlight_keywords() {
        let code = "fn main() { let x = 5; }";
        let result = highlight_rust(code);
        assert!(result.contains("hl-keyword"));
    }

    #[test]
    fn test_highlight_strings() {
        let code = r#"let s = "hello world";"#;
        let result = highlight_rust(code);
        assert!(result.contains("hl-string"));
    }

    #[test]
    fn test_highlight_raw_strings() {
        let code = "let s = r#\"raw string\"#;";
        let result = highlight_rust(code);
        assert!(result.contains("hl-string"));
    }

    #[test]
    fn test_highlight_comments() {
        let code = "// this is a comment\nlet x = 1;";
        let result = highlight_rust(code);
        assert!(result.contains("hl-comment"));
    }

    #[test]
    fn test_highlight_numbers() {
        let code = "let x = 42; let y = 3.14;";
        let result = highlight_rust(code);
        assert!(result.contains("hl-number"));
    }

    #[test]
    fn test_highlight_macros() {
        let code = "println!(\"hello\");";
        let result = highlight_rust(code);
        assert!(result.contains("hl-macro"));
    }
}
