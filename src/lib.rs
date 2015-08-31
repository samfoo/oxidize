#![feature(box_syntax)]
#![feature(core)]
extern crate core;
use core::fmt::Debug;

pub struct Expectation<Lhs: Debug>(Lhs);

pub struct To<Lhs: Debug>(Lhs);

pub trait Matcher<Lhs> {
    fn matches(&self, rhs: &Lhs) -> bool;
    fn fail_msg(&self, rhs: &Lhs) -> String;
}

pub trait Collection {
    fn match_len(&self) -> usize;
}

impl<T> Collection for Vec<T> {
    fn match_len(&self) -> usize { self.len() }
}

impl Collection for String {
    fn match_len(&self) -> usize { self.chars().count() }
}

impl<'a> Collection for &'a str {
    fn match_len(&self) -> usize { self.chars().count() }
}

pub struct Contains<T>(T);

impl<T: Debug + PartialEq> Matcher<Vec<T>> for Contains<T> {
    fn matches(&self, lhs: &Vec<T>) -> bool {
        lhs.iter().any(|i| *i == self.0)
    }

    fn fail_msg(&self, lhs: &Vec<T>) -> String {
        format!("expected {:?} to contain {:?}", lhs, self.0)
    }
}

impl Matcher<String> for Contains<char> {
    fn matches(&self, lhs: &String) -> bool {
        lhs.chars().any(|i| i == self.0)
    }

    fn fail_msg(&self, lhs: &String) -> String {
        format!("expected {:?} to contain {:?}", lhs, self.0)
    }
}

impl Matcher<String> for Contains<String> {
    fn matches(&self, lhs: &String) -> bool {
        lhs.contains(&*self.0)
    }

    fn fail_msg(&self, lhs: &String) -> String {
        format!("expected {:?} to contain {:?}", lhs, self.0)
    }
}

pub struct Empty;

impl<Lhs: Debug + Collection> Matcher<Lhs> for Empty {
    fn matches(&self, rhs: &Lhs) -> bool {
        rhs.match_len() == 0
    }

    fn fail_msg(&self, rhs: &Lhs) -> String {
        format!("expected {:?} to be empty", rhs)
    }
}

pub struct LessThan<Lhs: Debug>(Lhs);

impl<Lhs: Debug + PartialOrd> Matcher<Lhs> for LessThan<Lhs> {
    fn matches(&self, lhs: &Lhs) -> bool {
        *lhs < self.0
    }

    fn fail_msg(&self, lhs: &Lhs) -> String {
        format!("expected {:?} to be less than {:?}", lhs, self.0)
    }
}

pub struct GreaterThan<Lhs: Debug>(Lhs);

impl<Lhs: Debug + PartialOrd> Matcher<Lhs> for GreaterThan<Lhs> {
    fn matches(&self, lhs: &Lhs) -> bool {
        *lhs > self.0
    }

    fn fail_msg(&self, lhs: &Lhs) -> String {
        format!("expected {:?} to be greater than {:?}", lhs, self.0)
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

pub struct AllOf<A: Debug>(Vec<Box<Matcher<A>>>);

impl<Lhs: Debug> Matcher<Lhs> for AllOf<Lhs> {
    fn matches(&self, lhs: &Lhs) -> bool {
        self.0.iter().all(|bm| bm.matches(lhs))
    }

    fn fail_msg(&self, lhs: &Lhs) -> String {
        format!("expected all of {:?}",
                self.0.iter().map(|m| m.fail_msg(lhs)).collect::<Vec<String>>())
    }
}

pub struct AnyOf<A: Debug>(Vec<Box<Matcher<A>>>);

impl<Lhs: Debug> Matcher<Lhs> for AnyOf<Lhs> {
    fn matches(&self, lhs: &Lhs) -> bool {
        self.0.iter().any(|bm| bm.matches(lhs))
    }

    fn fail_msg(&self, lhs: &Lhs) -> String {
        format!("expected one of {:?}",
                self.0.iter().map(|m| m.fail_msg(lhs)).collect::<Vec<String>>())
    }
}

pub struct Not<A>(Box<Matcher<A>>);

impl<Lhs: Debug> Matcher<Lhs> for Not<Lhs> {
    fn matches(&self, rhs: &Lhs) -> bool {
        !self.0.matches(rhs)
    }

    fn fail_msg(&self, rhs: &Lhs) -> String {
        format!("not {}", self.0.fail_msg(rhs))
    }
}

impl<Lhs: Debug> Expectation<Lhs> {
    pub fn is<T>(&self, matcher: Box<T>) where T: Matcher<Lhs> {
        self.to(matcher)
    }

    pub fn to<T>(&self, matcher: Box<T>) where T: Matcher<Lhs> {
        if !matcher.matches(&self.0) {
            panic!(matcher.fail_msg(&self.0))
        }
    }
}

pub fn expect<T: Debug>(lhs: T) -> Expectation<T> {
    Expectation(lhs)
}

pub fn equal<T: Debug>(rhs: T) -> Box<Equal<T>> {
    box Equal(rhs)
}

pub fn not<T: Debug>(rhs: Box<Matcher<T>>) -> Box<Not<T>> {
    box Not(rhs)
}

pub fn empty() -> Box<Empty> {
    box Empty
}

pub fn contain<T: Debug>(rhs: T) -> Box<Contains<T>> {
    box Contains(rhs)
}

pub fn greater_than<T: Debug>(rhs: T) -> Box<GreaterThan<T>> {
    box GreaterThan(rhs)
}

pub fn less_than<T: Debug>(rhs: T) -> Box<LessThan<T>> {
    box LessThan(rhs)
}

pub fn any_of<T: Debug>(rhs: Vec<Box<Matcher<T>>>) -> Box<AnyOf<T>> {
    box AnyOf(rhs)
}

pub fn all_of<T: Debug>(rhs: Vec<Box<Matcher<T>>>) -> Box<AllOf<T>> {
    box AllOf(rhs)
}

#[cfg(test)]
mod test {
    use super::*;

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
    #[should_panic(expected="expected [1, 2, 3] to be empty")]
    fn test_not_empty_vec_fails() {
        expect(vec![1, 2, 3]).is(empty());
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
    #[should_panic(expected="expected \"hey diddle diddle\" to be empty")]
    fn test_not_empty_str_fails() {
        expect("hey diddle diddle").is(empty())
    }

    #[test]
    fn test_not_empty_str() {
        expect("not-empty").is(not(empty()));
    }

    #[test]
    fn test_contains() {
        expect(vec![1, 2, 3]).to(contain(1));
    }

    #[test]
    fn test_not_contains() {
        expect(vec![1, 2, 3]).to(not(contain(10)));
    }

    #[test]
    #[should_panic(expected="expected [1, 2, 3] to contain 5")]
    fn test_not_contains_fails() {
        expect(vec![1, 2, 3]).to(contain(5));
    }

    #[test]
    fn test_contains_string_char() {
        expect("Hello, world!".to_string()).to(contain('H'));
    }

    #[test]
    fn test_not_contains_string_char() {
        expect("Hello, world!".to_string()).to(not(contain('Z')));
    }

    #[test]
    #[should_panic(expected="expected \"fe fi fo fum\" to contain 'z'")]
    fn test_not_contains_char_fails() {
        expect("fe fi fo fum".to_string()).to(contain('z'));
    }

    #[test]
    fn test_contains_string_substring() {
        expect("Hello, world!".to_string()).to(contain("Hello".to_string()));
    }

    #[test]
    fn test_not_contains_string_substring() {
        expect("Hello, world!".to_string()).to(not(contain("not-in-there".to_string())));
    }

    #[test]
    #[should_panic(expected="expected \"fe fi fo fum\" to contain \"substring\"")]
    fn test_not_contains_substring_fails() {
        expect("fe fi fo fum".to_string()).to(contain("substring".to_string()));
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

    #[test]
    fn test_any_of() {
        expect(5).is(any_of(vec![less_than(0),
                                 greater_than(1)]))
    }

    #[test]
    fn test_any_of_none() {
        expect(5).is(not(any_of(vec![less_than(0),
                                     greater_than(100)])))
    }

    #[test]
    fn test_all_of() {
        expect(5).is(all_of(vec![less_than(1000),
                                 greater_than(1)]))
    }

    #[test]
    fn test_not_all_of() {
        expect(5).is(not(all_of(vec![less_than(1000),
                                     greater_than(10)])))
    }
}
