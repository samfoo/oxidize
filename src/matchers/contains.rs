use core::fmt::Debug;
use super::Matcher;

pub struct Contains<T>(pub T);

impl<T: Debug + PartialEq> Matcher<Vec<T>> for Contains<T> {
    fn matches(&self, lhs: &Vec<T>) -> bool {
        lhs.iter().any(|i| *i == self.0)
    }

    fn fail_msg(&self, lhs: &Vec<T>) -> String {
        format!("expected {:?} to contain {:?}", lhs, self.0)
    }
}

impl Matcher<String> for Contains<char> {
    fn matches(&self, lhs: &String) -> bool {
        lhs.chars().any(|i| i == self.0)
    }

    fn fail_msg(&self, lhs: &String) -> String {
        format!("expected {:?} to contain {:?}", lhs, self.0)
    }
}

impl Matcher<String> for Contains<String> {
    fn matches(&self, lhs: &String) -> bool {
        lhs.contains(&*self.0)
    }

    fn fail_msg(&self, lhs: &String) -> String {
        format!("expected {:?} to contain {:?}", lhs, self.0)
    }
}

#[cfg(test)]
mod test {
    use super::super::super::dsl::*;

    #[test]
    fn test_contains() {
        expect(vec![1, 2, 3]).to(contain(1));
    }

    #[test]
    fn test_not_contains() {
        expect(vec![1, 2, 3]).to(not(contain(10)));
    }

    #[test]
    #[should_panic(expected="expected [1, 2, 3] to contain 5")]
    fn test_not_contains_fails() {
        expect(vec![1, 2, 3]).to(contain(5));
    }

    #[test]
    fn test_contains_string_char() {
        expect("Hello, world!".to_string()).to(contain('H'));
    }

    #[test]
    fn test_not_contains_string_char() {
        expect("Hello, world!".to_string()).to(not(contain('Z')));
    }

    #[test]
    #[should_panic(expected="expected \"fe fi fo fum\" to contain 'z'")]
    fn test_not_contains_char_fails() {
        expect("fe fi fo fum".to_string()).to(contain('z'));
    }

    #[test]
    fn test_contains_string_substring() {
        expect("Hello, world!".to_string()).to(contain("Hello".to_string()));
    }

    #[test]
    fn test_not_contains_string_substring() {
        expect("Hello, world!".to_string()).to(not(contain("not-in-there".to_string())));
    }

    #[test]
    #[should_panic(expected="expected \"fe fi fo fum\" to contain \"substring\"")]
    fn test_not_contains_substring_fails() {
        expect("fe fi fo fum".to_string()).to(contain("substring".to_string()));
    }

}

