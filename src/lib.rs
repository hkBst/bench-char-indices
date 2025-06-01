use std::ops::Range;

#[derive(Debug, PartialEq, Eq)]
pub enum EscapeError {
    /// Raw '\r' encountered.
    BareCarriageReturn,
    /// Raw '\r' encountered in raw string.
    BareCarriageReturnInRawString,
}

pub fn check_raw_str_while(
    src: &str,
    mut callback: impl FnMut(Range<usize>, Result<char, EscapeError>),
) {
    let mut chars = src.chars();

    while let Some(c) = chars.next() {
        let start = src.len() - chars.as_str().len() - c.len_utf8();
        let res = match c {
            '\r' => Err(EscapeError::BareCarriageReturn),
            _ => Ok(c),
        };
        let end = src.len() - chars.as_str().len();
        callback(start..end, res);
    }
}

pub fn check_raw_str_char_indices(
    src: &str,
    mut callback: impl FnMut(Range<usize>, Result<char, EscapeError>),
) {
    src.char_indices().for_each(|(pos, c)| {
        callback(
            pos..pos + c.len_utf8(),
            if c == '\r' {
                Err(EscapeError::BareCarriageReturn)
            } else {
                Ok(c)
            },
        );
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same() {
        let s = "abcdefghijklmnopqrstuvwxyz0123456789";
        let (mut r1, mut r2) = (vec![], vec![]);
        check_raw_str_while(&s, |r, c| r1.push((r, c)));
        check_raw_str_char_indices(&s, |r, c| r2.push((r, c)));
        assert_eq!(r1, r2);
    }
}
