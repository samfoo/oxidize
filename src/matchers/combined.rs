use std::fmt::Debug;
use super::Matcher;

pub struct AllOf<A: Debug>(pub Vec<Box<Matcher<A>>>);

impl<Lhs: Debug> Matcher<Lhs> for AllOf<Lhs> {
    fn matches(&self, lhs: &Lhs) -> bool {
        self.0.iter().all(|bm| bm.matches(lhs))
    }

    fn fail_msg(&self, lhs: &Lhs) -> String {
        format!("expected all of {:?}",
                self.0.iter().map(|m| m.fail_msg(lhs)).collect::<Vec<String>>())
    }
}

pub struct AnyOf<A: Debug>(pub Vec<Box<Matcher<A>>>);

impl<Lhs: Debug> Matcher<Lhs> for AnyOf<Lhs> {
    fn matches(&self, lhs: &Lhs) -> bool {
        self.0.iter().any(|bm| bm.matches(lhs))
    }

    fn fail_msg(&self, lhs: &Lhs) -> String {
        format!("expected one of {:?}",
                self.0.iter().map(|m| m.fail_msg(lhs)).collect::<Vec<String>>())
    }
}

#[cfg(test)]
mod test {
    use super::super::super::dsl::*;

    #[test]
    fn test_any_of() {
        expect(5).is(any_of(vec![less_than(0),
                                 greater_than(1)]))
    }

    #[test]
    #[should_panic(expected="expected one of [\"expected 5 to be less than 0\", \"expected 5 to be greater than 100\"]")]
    fn test_any_of_none() {
        expect(5).is(any_of(vec![less_than(0),
                                 greater_than(100)]))
    }

    #[test]
    fn test_all_of() {
        expect(5).is(all_of(vec![less_than(1000),
                                 greater_than(1)]))
    }

    #[test]
    #[should_panic(expected="expected all of [\"expected 5 to be less than 1000\", \"expected 5 to be greater than 10\"]")]
    fn test_not_all_of() {
        expect(5).is(all_of(vec![less_than(1000),
                                 greater_than(10)]))
    }
}
