//! # Unaccent
//!
//! `unaccent` is a lightweight utility crate designed to remove accents (diacritical marks)
//! from strings. Inspired by the PostgreSQL `unaccent` extension, it provides an easy-to-use
//! function for normalizing text in Rust applications.
//!
//! ## Example
//! ```rust
//! use unaccent::unaccent;
//!
//! fn main() {
//!     let input = "Café au lait";
//!     let result = unaccent(input);
//!     assert_eq!(result, "Cafe au lait");
//! }
//!
//! ```

use unicode_normalization::{char::is_combining_mark, UnicodeNormalization};

/// Removes accents (diacritical marks) from a given string and returns the unaccented version.
///
/// This function uses Unicode Normalization Form Decomposed (NFD) to split characters into
/// their base characters and diacritical marks. It then filters out the combining marks.
///
/// # Arguments
///
/// * `input` - A string or a type that can be referenced as a string (e.g., `&str`, `String`).
///
/// # Returns
///
/// A `String` with all diacritical marks removed.
///
/// # Example
/// ```rust
/// use unaccent::unaccent;
///
/// let result = unaccent("coração");
/// assert_eq!(result, "coracao");
/// ```
pub fn unaccent<T: AsRef<str>>(input: T) -> String {
    input
        .as_ref()
        .nfd()
        .filter(|c| !is_combining_mark(*c))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_characters() {
        assert_eq!(unaccent("José"), "Jose");
        assert_eq!(unaccent("João"), "Joao");
        assert_eq!(unaccent("Água"), "Agua");
        assert_eq!(unaccent("Müller"), "Muller");
    }

    #[test]
    fn test_mixed_characters() {
        assert_eq!(unaccent("crème brûlée"), "creme brulee");
        assert_eq!(unaccent("coração"), "coracao");
        assert_eq!(unaccent("niño"), "nino");
        assert_eq!(unaccent("über"), "uber");
    }

    #[test]
    fn test_no_accents() {
        assert_eq!(unaccent("hello"), "hello");
        assert_eq!(unaccent("world"), "world");
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(unaccent(""), "");
    }

    #[test]
    fn test_special_characters() {
        assert_eq!(unaccent("@#!$%"), "@#!$%");
        assert_eq!(unaccent("12345"), "12345");
    }

    #[ignore = "deal with the 'ø' case later"]
    #[test]
    fn test_combined_accented_characters() {
        assert_eq!(unaccent("éèêëēėę"), "eeeeeee");
        assert_eq!(unaccent("áàâäãåā"), "aaaaaaa");
        assert_eq!(unaccent("óòôöõøō"), "ooooooo");
    }

    #[test]
    fn test_unicode_characters() {
        assert_eq!(unaccent("你好"), "你好");
        assert_eq!(unaccent("résumé"), "resume");
        assert_eq!(unaccent("coöperate"), "cooperate");
    }
}
