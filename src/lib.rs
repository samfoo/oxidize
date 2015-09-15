#![feature(core)]

use matchers::Matcher;
use dsl::*;

mod matchers;
mod dsl;

extern crate core;
use core::fmt::Debug;

pub struct Expectation<Lhs: Debug>(Lhs);

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

