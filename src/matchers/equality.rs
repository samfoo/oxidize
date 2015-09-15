use std::fmt::Debug;
use super::Matcher;

pub struct LessThan<Lhs: Debug>(pub Lhs);

impl<Lhs: Debug + PartialOrd> Matcher<Lhs> for LessThan<Lhs> {
    fn matches(&self, lhs: &Lhs) -> bool {
        *lhs < self.0
    }

    fn fail_msg(&self, lhs: &Lhs) -> String {
        format!("expected {:?} to be less than {:?}", lhs, self.0)
    }
}

pub struct GreaterThan<Lhs: Debug>(pub Lhs);

impl<Lhs: Debug + PartialOrd> Matcher<Lhs> for GreaterThan<Lhs> {
    fn matches(&self, lhs: &Lhs) -> bool {
        *lhs > self.0
    }

    fn fail_msg(&self, lhs: &Lhs) -> String {
        format!("expected {:?} to be greater than {:?}", lhs, self.0)
    }
}

pub struct Equal<Lhs: Debug>(pub Lhs);

impl<Lhs: Debug + PartialEq> Matcher<Lhs> for Equal<Lhs> {
    fn matches(&self, rhs: &Lhs) -> bool {
        &self.0 == rhs
    }

    fn fail_msg(&self, rhs: &Lhs) -> String {
        format!("expected {:?} to equal {:?}", rhs, self.0)
    }
}

#[cfg(test)]
mod test {
    use super::super::super::dsl::*;

    #[test]
    fn test_expect_equality() {
        expect(1).to(equal(1));
    }

    #[test]
    #[should_panic(expected="expected 1 to equal 2")]
    fn test_expect_not_equality_to_fail() {
        expect(1).to(equal(2));
    }

    #[test]
    fn test_partial_eq_equality() {
        #[derive(PartialEq, Debug)]
        struct Foo { a: u16, b: String }

        expect(Foo{ a: 10u16, b: "Hello".to_string()})
            .to(equal(Foo{ a: 10u16, b: "Hello".to_string()}));
    }

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

    #[test]
    fn test_greater_than_int() {
        expect(5).is(greater_than(1));
    }

    #[test]
    fn test_not_greater_than_int() {
        expect(5).is(not(greater_than(10)));
    }

    #[test]
    #[should_panic(expected="expected 5 to be greater than 6")]
    fn test_not_greater_than_fails() {
        expect(5).is(greater_than(6));
    }

    #[test]
    fn test_less_than_int() {
        expect(5).is(less_than(10));
    }

    #[test]
    fn test_not_less_than_int() {
        expect(5).is(not(less_than(1)));
    }

    #[test]
    #[should_panic(expected="expected 10 to be less than 6")]
    fn test_not_less_than_fails() {
        expect(10).is(less_than(6));
    }

}
