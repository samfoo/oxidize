use std::fmt::Debug;
use super::Matcher;

pub struct Contains<T>(pub T);

impl<T: Debug + PartialEq> Matcher<Vec<T>> for Contains<T> {
    fn matches(&self, lhs: &Vec<T>) -> bool {
        lhs.iter().any(|i| *i == self.0)
    }

    fn fail_msg(&self, lhs: &Vec<T>) -> String {
        format!("expected {:?} to contain {:?}", lhs, self.0)
    }

    fn negated_fail_msg(&self, lhs: &Vec<T>) -> String {
        format!("expected {:?} not to contain {:?}", lhs, self.0)
    }
}

impl Matcher<String> for Contains<char> {
    fn matches(&self, lhs: &String) -> bool {
        lhs.chars().any(|i| i == self.0)
    }

    fn fail_msg(&self, lhs: &String) -> String {
        format!("expected {:?} to contain {:?}", lhs, self.0)
    }

    fn negated_fail_msg(&self, lhs: &String) -> String {
        format!("expected {:?} not to contain {:?}", lhs, self.0)
    }
}

impl Matcher<String> for Contains<String> {
    fn matches(&self, lhs: &String) -> bool {
        lhs.contains(&*self.0)
    }

    fn fail_msg(&self, lhs: &String) -> String {
        format!("expected {:?} to contain {:?}", lhs, self.0)
    }

    fn negated_fail_msg(&self, lhs: &String) -> String {
        format!("expected {:?} not to contain {:?}", lhs, self.0)
    }
}

impl<'a> Matcher<String> for Contains<&'a str> {
    fn matches(&self, lhs: &String) -> bool {
        lhs.contains(self.0)
    }

    fn fail_msg(&self, lhs: &String) -> String {
        format!("expected {:?} to contain {:?}", lhs, self.0)
    }

    fn negated_fail_msg(&self, lhs: &String) -> String {
        format!("expected {:?} not to contain {:?}", lhs, self.0)
    }
}

impl<'a> Matcher<&'a str> for Contains<String> {
    fn matches(&self, lhs: &&'a str) -> bool {
        (*lhs).contains(&*self.0)
    }

    fn fail_msg(&self, lhs: &&'a str) -> String {
        format!("expected {:?} to contain {:?}", lhs, self.0)
    }

    fn negated_fail_msg(&self, lhs: &&'a str) -> String {
        format!("expected {:?} not to contain {:?}", lhs, self.0)
    }
}

impl<'a, 'b> Matcher<&'a str> for Contains<&'b str> {
    fn matches(&self, lhs: &&'a str) -> bool {
        let lhs = *lhs;
        let rhs = self.0;

        lhs.contains(rhs)
    }

    fn fail_msg(&self, lhs: &&'a str) -> String {
        format!("expected {:?} to contain {:?}", lhs, self.0)
    }

    fn negated_fail_msg(&self, lhs: &&'a str) -> String {
        format!("expected {:?} not to contain {:?}", lhs, self.0)
    }
}

#[cfg(test)]
mod test {
    mod vec_t_contains_t {
        use super::super::super::super::dsl::*;

        #[test]
        fn test_contains_with_vector_matches() {
            expect(vec![1, 2, 3]).to(contain(1));
        }

        #[test]
        #[should_panic(expected="expected [1, 2, 3] to contain 5")]
        fn test_contains_with_vector_fails_with_message() {
            expect(vec![1, 2, 3]).to(contain(5));
        }

        #[test]
        #[should_panic(expected="expected [1, 2, 3] not to contain 2")]
        fn test_negated_contains_with_vector_fails_with_message() {
            expect(vec![1, 2, 3]).to_not(contain(2));
        }
    }

    mod str_contains_char {
        use super::super::super::super::dsl::*;

        #[test]
        fn test_contains_char_in_str_matches() {
            expect("Hello, world!".to_string()).to(contain('H'));
        }

        #[test]
        #[should_panic(expected="expected \"fe fi fo fum\" to contain 'z'")]
        fn test_contains_char_in_str_fails_with_message() {
            expect("fe fi fo fum".to_string()).to(contain('z'));
        }

        #[test]
        #[should_panic(expected="expected \"fe fi fo fum\" not to contain 'f'")]
        fn test_negated_contains_char_in_str_fails_with_message() {
            expect("fe fi fo fum".to_string()).to_not(contain('f'));
        }
    }

    mod str_contains_str {
        use super::super::super::super::dsl::*;

        #[test]
        fn test_contains_substring_in_str_matches() {
            expect("Hello, world!").to(contain("Hello"));
        }

        #[test]
        #[should_panic(expected="expected \"Hello, world!\" to contain \"not-in-there\"")]
        fn test_contains_substring_in_str_fails_with_message() {
            expect("Hello, world!").to(contain("not-in-there"));
        }

        #[test]
        #[should_panic(expected="expected \"Hello, world!\" not to contain \"Hello\"")]
        fn test_negated_contains_substring_in_str_fails_with_message() {
            expect("Hello, world!").to_not(contain("Hello"));
        }
    }

    mod string_contains_str {
        use super::super::super::super::dsl::*;

        #[test]
        fn test_contains_substring_in_string_matches() {
            expect("Hello, world!".to_string()).to(contain("Hello"));
        }

        #[test]
        #[should_panic(expected="expected \"Hello, world!\" to contain \"not-in-there\"")]
        fn test_contains_substring_in_string_fails_with_message() {
            expect("Hello, world!".to_string()).to(contain("not-in-there"));
        }

        #[test]
        #[should_panic(expected="expected \"Hello, world!\" not to contain \"Hello\"")]
        fn test_negated_contains_substring_in_string_fails_with_message() {
            expect("Hello, world!".to_string()).to_not(contain("Hello"));
        }
    }

    mod str_contains_string {
        use super::super::super::super::dsl::*;

        #[test]
        fn test_contains_string_substring_in_str_matches() {
            expect("Hello, world!").to(contain("Hello".to_string()));
        }

        #[test]
        #[should_panic(expected="expected \"Hello, world!\" to contain \"not-in-there\"")]
        fn test_contains_string_substring_in_str_fails_with_message() {
            expect("Hello, world!").to(contain("not-in-there".to_string()));
        }

        #[test]
        #[should_panic(expected="expected \"Hello, world!\" not to contain \"Hello\"")]
        fn test_negated_contains_string_substring_in_str_fails_with_message() {
            expect("Hello, world!").to_not(contain("Hello".to_string()));
        }
    }

    mod string_contains_string {
        use super::super::super::super::dsl::*;

        #[test]
        fn test_contains_string_substring_in_string_matches() {
            expect("Hello, world!".to_string()).to(contain("Hello".to_string()));
        }

        #[test]
        #[should_panic(expected="expected \"fe fi fo fum\" to contain \"substring\"")]
        fn test_contains_string_substring_in_string_fails_with_message() {
            expect("fe fi fo fum".to_string()).to(contain("substring".to_string()));
        }

        #[test]
        #[should_panic(expected="expected \"fe fi fo fum\" not to contain \"fum\"")]
        fn test_negated_contains_string_substring_in_string_fails_with_message() {
            expect("fe fi fo fum".to_string()).to_not(contain("fum".to_string()));
        }
    }
}

