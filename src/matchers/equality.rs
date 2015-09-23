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

    fn negated_fail_msg(&self, lhs: &Lhs) -> String {
        format!("expected {:?} to be greater than or equal to {:?}", lhs, self.0)
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

    fn negated_fail_msg(&self, lhs: &Lhs) -> String {
        format!("expected {:?} to be less than or equal to {:?}", lhs, self.0)
    }
}

pub struct Equal<Lhs: Debug>(pub Lhs);

impl<Lhs: Debug + PartialEq> Matcher<Lhs> for Equal<Lhs> {
    fn matches(&self, rhs: &Lhs) -> bool {
        &self.0 == rhs
    }

    fn fail_msg(&self, rhs: &Lhs) -> String {
        format!("\nexpected: {:?}\n     got: {:?}\n", rhs, self.0)
    }

    fn negated_fail_msg(&self, lhs: &Lhs) -> String {
        format!("expected {:?} not to equal {:?}", lhs, self.0)
    }
}

#[cfg(test)]
mod test {
    mod eq {
        use super::super::super::super::dsl::*;

        #[test]
        fn test_equal_matches() {
            expect(1).to(equal(1));
        }

        #[test]
        #[should_panic(expected="\nexpected: 1\n     got: 2\n")]
        fn test_equal_fails_with_message() {
            expect(1).to(equal(2));
        }

        #[test]
        #[should_panic(expected="expected 1 not to equal 1")]
        fn test_negated_equal_fails_with_message() {
            expect(1).to_not(equal(1));
        }

        #[test]
        fn test_equal_with_partial_equal_matches() {
            #[derive(PartialEq, Debug)]
            struct Foo { a: u16, b: String }

            expect(Foo{ a: 10u16, b: "Hello".to_string()})
                .to(equal(Foo{ a: 10u16, b: "Hello".to_string()}));
        }

        #[test]
        #[should_panic(expected="\nexpected: Foo { a: 10, b: \"Hello\" }\n     got: Foo { a: 10, b: \"Goodbye\" }\n")]
        fn test_equal_with_partial_equal_fails_with_message() {
            #[derive(PartialEq, Debug)]
            struct Foo { a: u16, b: String }

            expect(Foo{ a: 10u16, b: "Hello".to_string()})
                .to(equal(Foo{ a: 10u16, b: "Goodbye".to_string()}));
        }
    }

    mod greater_than {
        use super::super::super::super::dsl::*;

        #[test]
        fn test_greater_than_matches() {
            expect(5).is(greater_than(1));
        }

        #[test]
        #[should_panic(expected="expected 5 to be greater than 6")]
        fn test_greater_than_fails_with_message() {
            expect(5).is(greater_than(6));
        }

        #[test]
        #[should_panic(expected="expected 5 to be less than or equal to 3")]
        fn test_negated_greater_than_fails_with_message() {
            expect(5).is_not(greater_than(3));
        }
    }

    mod less_than {
        use super::super::super::super::dsl::*;

        #[test]
        fn test_less_than_matches() {
            expect(5).is(less_than(10));
        }

        #[test]
        #[should_panic(expected="expected 10 to be less than 6")]
        fn test_less_than_fails_with_message() {
            expect(10).is(less_than(6));
        }

        #[test]
        #[should_panic(expected="expected 10 to be greater than or equal to 20")]
        fn test_negated_less_than_fails_with_message() {
            expect(10).is_not(less_than(20));
        }
    }
}
