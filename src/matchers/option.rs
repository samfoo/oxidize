use std::fmt::Debug;
use std::intrinsics::type_name;
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

    fn negated_fail_msg(&self, lhs: &Option<T>) -> String {
        format!("expected {:?} to be Some<{}>", lhs, unsafe { type_name::<T>() })
    }
}

impl<T: Debug> Matcher<Option<T>> for Something {
    fn matches(&self, lhs: &Option<T>) -> bool {
        lhs.is_some()
    }

    fn fail_msg(&self, lhs: &Option<T>) -> String {
        format!("expected {:?} to be Some<{}>", lhs, unsafe { type_name::<T>() })
    }

    fn negated_fail_msg(&self, lhs: &Option<T>) -> String {
        format!("expected {:?} to be None", lhs)
    }
}

#[cfg(test)]
mod test {
    use super::super::super::dsl::*;

    #[test]
    fn test_none_matches() {
        let n: Option<String> = None;
        expect(n).is(none());
    }

    #[test]
    #[should_panic(expected="expected Some(\"Hello\") to be None")]
    fn test_none_fails_with_message() {
        let n = Some("Hello");
        expect(n).is(none());
    }

    #[test]
    #[should_panic(expected="expected None to be Some<i32>")]
    fn test_negated_none_fails_with_message() {
        let n: Option<i32> = None;
        expect(n).is_not(none());
    }

    #[test]
    fn test_some_matches() {
        let n = Some(32);
        expect(n).is(some());
    }

    #[test]
    #[should_panic(expected="expected None to be Some<i32>")]
    fn test_some_fails_with_message() {
        let n: Option<i32> = None;
        expect(n).is(some());
    }

    #[test]
    #[should_panic(expected="expected Some(32) to be None")]
    fn test_negated_some_fails_with_message() {
        let n = Some(32);
        expect(n).is_not(some());
    }
}
