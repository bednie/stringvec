//! # stringvec
//!
//! `stringvec` is a Rust library that provides a macro for easily creating
//! a `Vec<String>` from various types, along with a utility function to check
//! if a value is a `String`.

use std::any::{Any, TypeId};

/// Creates a `Vec<String>` from a list of expressions.
///
/// This macro converts each expression to a `String` using the `to_string()`
/// method and collects them into a `Vec<String>`.
///
/// # Examples
///
/// ```
/// use stringvec::stringvec;
///
/// let words = stringvec!["hello", 42, 'A', 3.14];
/// assert_eq!(words, vec!["hello", "42", "A", "3.14"]);
/// ```
#[macro_export]
macro_rules! stringvec {
    ($($element:expr),* $(,)?) => {
        vec![$($element.to_string()),*]
    };
}

/// Checks if the provided value is a `String`.
///
/// # Examples
///
/// ```
/// use stringvec::is_string;
///
/// let s = "hello".to_string();
/// let i = 42;
///
/// assert!(is_string(&s));
/// assert!(!is_string(&i));
/// ```
pub fn is_string(s: &dyn Any) -> bool {
    TypeId::of::<String>() == s.type_id()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stringvec_macro() {
        let words = stringvec!["cat", 11, 'A', 3.5];
        assert_eq!(words, vec!["cat", "11", "A", "3.5"]);
    }

    #[test]
    fn test_is_string() {
        let s = "Hello".to_string();
        let i = 42;
        assert!(is_string(&s));
        assert!(!is_string(&i));
    }
}