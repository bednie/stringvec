use std::any::{Any, TypeId};

   #[macro_export]
   macro_rules! stringvec {
       ($($element:expr),* $(,)?) => {
           vec![$($element.to_string()),*]
       };
   }

   pub fn is_string(s: &dyn Any) -> bool {
       TypeId::of::<String>() == s.type_id()
   }

   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_svec_macro() {
           let words = stringvec!["cat", 11, 'A', 3.14];
           assert_eq!(words, vec!["cat", "11", "A", "3.14"]);
       }

       #[test]
       fn test_is_string() {
           let s = "Hello".to_string();
           let i = 42;
           assert!(is_string(&s));
           assert!(!is_string(&i));
       }
   }