/// A basic `clsx`-style utility that joins non-None, non-empty strings with a space.
pub fn cn(classes: &[Option<&str>]) -> String {
    classes
        .iter()
        .flatten()
        .copied()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_concat() {
        assert_eq!(cn(&[Some("a"), Some("b")]), "a b");
    }

    #[test]
    fn test_with_nones() {
        let active = true;
        assert_eq!(
            cn(&[Some("base"), if active { Some("active") } else { None }]),
            "base active"
        );
    }

    #[test]
    fn test_all_none() {
        assert_eq!(cn(&[None, None]), "");
    }

    #[test]
    fn test_partial() {
        let active = false;
        assert_eq!(
            cn(&[Some("base"), if active { Some("active") } else { None }, Some("static")]),
            "base static"
        );
    }

    #[test]
    fn test_empty_str() {
        assert_eq!(cn(&[Some(""), Some("b")]), "b");
    }
}
