use std::fmt::Debug;
use super::Matcher;

pub struct Nothing;
pub struct Something;

impl<T: Debug> Matcher<Option<T>> for Nothing {
    fn matches(&self, lhs: &Option<T>) -> bool {
        lhs.is_none()
    }

    fn fail_msg(&self, lhs: &Option<T>) -> String {
        format!("expected {:?} to be None", lhs)
    }
}

impl<T: Debug> Matcher<Option<T>> for Something {
    fn matches(&self, lhs: &Option<T>) -> bool {
        lhs.is_some()
    }

    fn fail_msg(&self, lhs: &Option<T>) -> String {
        format!("expected {:?} to be Some<_>", lhs)
    }
}

#[cfg(test)]
mod test {
    use super::super::super::dsl::*;

    #[test]
    fn test_none() {
        let n: Option<String> = None;
        expect(n).is(none());
    }

    #[test]
    #[should_panic(expected="expected Some(\"Hello\") to be None")]
    fn test_not_none_to_fail() {
        let n = Some("Hello");
        expect(n).is(none());
    }

    #[test]
    fn test_some() {
        let n = Some(32);
        expect(n).is(some());
    }

    #[test]
    #[should_panic(expected="expected None to be Some<_>")]
    fn test_not_some_to_fail() {
        let n: Option<i32> = None;
        expect(n).is(some());
    }
}
