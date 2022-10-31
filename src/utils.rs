pub fn is_digit(digit: u8) -> bool {
    digit >= b'0' && digit <= b'9'
}

pub fn is_alpha(alpha: u8) -> bool {
    (alpha >= b'a' && alpha <= b'z') || (alpha >= b'A' && alpha <= b'Z') || (alpha == b'_')
}

pub fn is_alphanumeric(c: u8) -> bool {
    is_digit(c) || is_alpha(c)
}
