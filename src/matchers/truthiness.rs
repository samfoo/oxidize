use super::Matcher;

pub struct BeTrue;
pub struct BeFalse;

impl Matcher<bool> for BeTrue {
    fn matches(&self, lhs: &bool) -> bool {
        *lhs
    }

    fn fail_msg(&self, lhs: &bool) -> String {
        format!("\nexpected: true\n     got: {:?}\n", lhs)
    }

    fn negated_fail_msg(&self, lhs: &bool) -> String {
        format!("\nexpected: false\n     got: {:?}\n", lhs)
    }
}

impl Matcher<bool> for BeFalse {
    fn matches(&self, lhs: &bool) -> bool {
        !*lhs
    }

    fn fail_msg(&self, lhs: &bool) -> String {
        format!("\nexpected: false\n     got: {:?}\n", lhs)
    }

    fn negated_fail_msg(&self, lhs: &bool) -> String {
        format!("\nexpected: true\n     got: {:?}\n", lhs)
    }
}

#[cfg(test)]
mod test {
    mod be_true {
        use super::super::super::super::dsl::*;

        #[test]
        fn test_be_true_matches() {
            expect(true).to(be_true());
        }

        #[test]
        #[should_panic(expected="\nexpected: true\n     got: false\n")]
        fn test_be_true_fails_with_message() {
            expect(false).to(be_true());
        }

        #[test]
        #[should_panic(expected="\nexpected: false\n     got: true\n")]
        fn test_negated_be_true_fails_with_message() {
            expect(true).to_not(be_true());
        }
    }

    mod be_false {
        use super::super::super::super::dsl::*;

        #[test]
        fn test_be_false_matches() {
            expect(false).to(be_false());
        }

        #[test]
        #[should_panic(expected="\nexpected: false\n     got: true\n")]
        fn test_be_false_fails_with_message() {
            expect(true).to(be_false());
        }

        #[test]
        #[should_panic(expected="\nexpected: true\n     got: false\n")]
        fn test_negated_be_false_fails_with_message() {
            expect(false).to_not(be_false());
        }
    }
}
