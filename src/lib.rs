#![feature(core)]
extern crate core;
use core::fmt::Debug;

pub struct Expectation<Lhs: Debug>(Lhs);

pub struct To<Lhs: Debug>(Lhs);

pub trait Matcher<Lhs: Debug> {
    fn matches(&self, rhs: &Lhs) -> bool;
    fn fail_msg(&self, rhs: &Lhs) -> String;
}

pub struct Equal<Lhs: Debug>(Lhs);

impl<Lhs: Debug + PartialEq> Matcher<Lhs> for Equal<Lhs> {
    fn matches(&self, rhs: &Lhs) -> bool {
        &self.0 == rhs
    }

    fn fail_msg(&self, rhs: &Lhs) -> String {
        format!("expected {:?} to equal {:?}", rhs, self.0)
    }
}

impl<Lhs: Debug> Expectation<Lhs> {
    fn to<T>(&self, matcher: T) where T: Matcher<Lhs> {
        if !matcher.matches(&self.0) {
            panic!(matcher.fail_msg(&self.0))
        }
    }
}

pub fn expect<T: Debug>(lhs: T) -> Expectation<T> {
    Expectation(lhs)
}

pub fn equal<T: Debug>(rhs: T) -> Equal<T> {
    Equal(rhs)
}

#[cfg(test)]
mod test {
    use super::{expect, equal};

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
}
