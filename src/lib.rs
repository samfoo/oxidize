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

pub trait WithLen {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

impl<T> WithLen for Vec<T> {
    fn len(&self) -> usize { self.len() }
    fn is_empty(&self) -> bool { self.is_empty() }
}

impl WithLen for String {
    fn len(&self) -> usize { self.chars().count() }
    fn is_empty(&self) -> bool { self.chars().count() == 0 }
}

impl<'a> WithLen for &'a str {
    fn len(&self) -> usize { self.chars().count() }
    fn is_empty(&self) -> bool { self.chars().count() == 0 }
}

pub struct Empty;
impl<Lhs: Debug + WithLen> Matcher<Lhs> for Empty {
    fn matches(&self, rhs: &Lhs) -> bool {
        rhs.is_empty()
    }

    fn fail_msg(&self, rhs: &Lhs) -> String {
        format!("expected {:?} to be empty", rhs)
    }
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
    pub fn is<T>(&self, matcher: T) where T: Matcher<Lhs> {
        self.to(matcher)
    }

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

pub fn empty() -> Empty {
    Empty
}

#[cfg(test)]
mod test {
    use super::{expect, equal, not, empty};

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
    fn test_empty() {
        let v: Vec<usize> = Vec::new();
        expect(v).is(empty());
    }

    #[test]
    fn test_not_empty() {
        let v = vec![1, 2, 3];
        expect(v).is(not(empty()));
    }


    #[test]
    fn test_empty_string() {
        expect("".to_string()).is(empty());
    }

    #[test]
    fn test_not_empty_string() {
        expect("not-empty".to_string()).is(not(empty()));
    }

    #[test]
    fn test_empty_str() {
        expect("").is(empty());
    }

    #[test]
    fn test_not_empty_str() {
        expect("not-empty").is(not(empty()));
    }
}
