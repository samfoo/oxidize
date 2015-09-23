use std::fmt::Debug;
use super::Matcher;

pub trait Collection {
    fn match_len(&self) -> usize;
}

impl<T> Collection for Vec<T> {
    fn match_len(&self) -> usize { self.len() }
}

impl Collection for String {
    fn match_len(&self) -> usize { self.chars().count() }
}

impl<'a> Collection for &'a str {
    fn match_len(&self) -> usize { self.chars().count() }
}


pub struct Empty;

impl<Lhs: Debug + Collection> Matcher<Lhs> for Empty {
    fn matches(&self, rhs: &Lhs) -> bool {
        rhs.match_len() == 0
    }

    fn fail_msg(&self, rhs: &Lhs) -> String {
        format!("expected {:?} to be empty", rhs)
    }

    fn negated_fail_msg(&self, rhs: &Lhs) -> String {
        format!("expected {:?} not to be empty", rhs)
    }
}

#[cfg(test)]
mod test {
    mod vec {
        use super::super::super::super::dsl::*;

        #[test]
        fn test_empty_vec_matches() {
            let v: Vec<usize> = Vec::new();
            expect(v).is(empty());
        }

        #[test]
        #[should_panic(expected="expected [1, 2, 3] to be empty")]
        fn test_empty_vec_fails_with_message() {
            expect(vec![1, 2, 3]).is(empty());
        }

        #[test]
        #[should_panic(expected="expected [] not to be empty")]
        fn test_negated_empty_vec_fails_with_message() {
            expect(Vec::<String>::new()).is_not(empty());
        }
    }

    mod string {
        use super::super::super::super::dsl::*;

        #[test]
        fn test_empty_string_matches() {
            expect("".to_string()).is(empty());
        }

        #[test]
        #[should_panic(expected="expected \"not-empty\" to be empty")]
        fn test_empty_string_fails_with_message() {
            expect("not-empty".to_string()).is(empty());
        }

        #[test]
        #[should_panic(expected="expected \"\" not to be empty")]
        fn test_negated_empty_string_fails_with_message() {
            expect("".to_string()).is_not(empty());
        }
    }

    mod str {
        use super::super::super::super::dsl::*;

        #[test]
        fn test_empty_str_matches() {
            expect("").is(empty());
        }

        #[test]
        #[should_panic(expected="expected \"hey diddle diddle\" to be empty")]
        fn test_empty_str_fails_with_message() {
            expect("hey diddle diddle").is(empty())
        }

        #[test]
        #[should_panic(expected="expected \"\" not to be empty")]
        fn test_negated_empty_str_fails_with_message() {
            expect("").is_not(empty())
        }
    }
}
