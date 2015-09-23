#![feature(core_intrinsics)]

use matchers::Matcher;

pub mod matchers;
pub mod dsl;

use std::fmt::Debug;

pub struct Expectation<Lhs: Debug>(Lhs);

impl<Lhs: Debug> Expectation<Lhs> {
    pub fn is<T>(&self, matcher: Box<T>) where T: Matcher<Lhs> {
        self.to(matcher)
    }

    pub fn is_not<T>(&self, matcher: Box<T>) where T: Matcher<Lhs> {
        self.to_not(matcher)
    }

    pub fn to<T>(&self, matcher: Box<T>) where T: Matcher<Lhs> {
        if !matcher.matches(&self.0) {
            panic!(matcher.fail_msg(&self.0))
        }
    }

    pub fn to_not<T>(&self, matcher: Box<T>) where T: Matcher<Lhs> {
        if matcher.matches(&self.0) {
            panic!(matcher.negated_fail_msg(&self.0))
        }
    }
}

