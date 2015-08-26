#![feature(core)]
extern crate core;
use core::fmt::Debug;
use std::marker::PhantomData;

pub struct Expectation<Lhs: Debug>(Lhs);

pub struct To<Lhs: Debug>(Lhs);

pub trait Matcher<Lhs> {
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

pub struct Not<A, M>(M, PhantomData<A>) where A: Debug, M: Matcher<A>;

impl<Lhs: Debug, M: Matcher<Lhs>> Matcher<Lhs> for Not<Lhs, M> {
    fn matches(&self, rhs: &Lhs) -> bool {
        !self.0.matches(rhs)
    }

    fn fail_msg(&self, rhs: &Lhs) -> String {
        format!("not {}", self.0.fail_msg(rhs))
    }
}

impl<Lhs: Debug> Expectation<Lhs> {
    pub fn to<T>(&self, matcher: T) where T: Matcher<Lhs> {
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

pub fn not<T: Debug, M: Matcher<T>>(rhs: M) -> Not<T, M> {
    Not(rhs, PhantomData)
}

#[cfg(test)]
mod test {
    use super::{expect, equal, not};

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
}
