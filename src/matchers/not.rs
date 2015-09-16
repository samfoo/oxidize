use std::fmt::Debug;
use super::Matcher;

pub struct Not<A>(pub Box<Matcher<A>>);

impl<Lhs: Debug> Matcher<Lhs> for Not<Lhs> {
    fn matches(&self, rhs: &Lhs) -> bool {
        !self.0.matches(rhs)
    }

    fn fail_msg(&self, rhs: &Lhs) -> String {
        format!("not {}", self.0.fail_msg(rhs))
    }
}

#[cfg(test)]
mod test {
    use super::super::super::dsl::*;

    #[test]
    fn test_invert_equality() {
        expect(1).to(not(equal(2)));
    }

    #[test]
    fn test_invert_invert_equality() {
        expect(1).to(not(not(equal(1))));
    }

    #[test]
    #[should_panic(expected="not expected 1 to equal 1")]
    fn test_invert_equality_equal_to_fail() {
        expect(1).to(not(equal(1)));
    }

}
