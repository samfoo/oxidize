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
}

#[cfg(test)]
mod test {
    use super::super::super::dsl::*;

    #[test]
    fn test_empty() {
        let v: Vec<usize> = Vec::new();
        expect(v).is(empty());
    }

    #[test]
    #[should_panic(expected="expected [1, 2, 3] to be empty")]
    fn test_not_empty_vec_fails() {
        expect(vec![1, 2, 3]).is(empty());
    }

    #[test]
    fn test_empty_string() {
        expect("".to_string()).is(empty());
    }

    #[test]
    #[should_panic(expected="expected \"not-empty\" to be empty")]
    fn test_not_empty_string() {
        expect("not-empty".to_string()).is(empty());
    }

    #[test]
    fn test_empty_str() {
        expect("").is(empty());
    }

    #[test]
    #[should_panic(expected="expected \"hey diddle diddle\" to be empty")]
    fn test_not_empty_str_fails() {
        expect("hey diddle diddle").is(empty())
    }
}
