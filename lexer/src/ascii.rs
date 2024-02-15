const DIGIT_MAP: u128 = 0b_00000000000000000000000000000000_00000000000000000000000000000000_00000011111111110000000000000000_00000000000000000000000000000000_u128;

/// check if u8 is digit
///
/// # Example
/// ```
/// use lexer::ascii::is_digit;
///
/// assert!(is_digit(b'0'));
/// assert!(is_digit(b'1'));
/// assert!(is_digit(b'2'));
/// assert!(is_digit(b'3'));
/// assert!(is_digit(b'4'));
/// assert!(is_digit(b'5'));
/// assert!(is_digit(b'6'));
/// assert!(is_digit(b'7'));
/// assert!(is_digit(b'8'));
/// assert!(is_digit(b'9'));
/// ```
pub fn is_digit(c: u8) -> bool {
    DIGIT_MAP & (1 << c) != 0
}

#[cfg(test)]
mod test {
    use crate::ascii::is_digit;

    #[test]
    fn check_is_digit_fn_test() {
        assert!(is_digit(b'0'));
        assert!(is_digit(b'1'));
        assert!(is_digit(b'2'));
        assert!(is_digit(b'3'));
        assert!(is_digit(b'4'));
        assert!(is_digit(b'5'));
        assert!(is_digit(b'6'));
        assert!(is_digit(b'7'));
        assert!(is_digit(b'8'));
        assert!(is_digit(b'9'));

        assert!(!is_digit(b'A'));
        assert!(!is_digit(b'k'));
        assert!(!is_digit(b'a'));
        assert!(!is_digit(b's'));
        assert!(!is_digit(b'h'));
    }
}
