#![doc(html_root_url="https://samfoo.github.io/oxidize/")]
#![cfg_attr(test, deny(warnings))]

//! # Oxidize
//!
//! Oxidize lets you set expected outcomes on objects in unit tests (or
//! anywhere). This allows for more fluid expression than with assertions alone
//! and yields more readable error messages.
//!
//! For example:
//!
//!     use oxidize::dsl::*;
//!
//!     expect("optimus prime").to(contain("prime"));
//!
//! Any expectation can also be negated:
//!
//!     use oxidize::dsl::*;
//!
//!     expect("megatron").to_not(contain("prime"));
//!
//! ## Built-in Matchers
//!
//! Oxidize has of built-in [matchers](matchers/index.html) that you can
//! use for all kinds of data.
//!
//! ### Equality
//!
//!     use oxidize::dsl::*;
//!
//!     expect(1).to(equal(1));
//!     expect(6).is(greater_than(0));
//!     expect(-120).is(less_than(5));
//!
//! ### Truthiness
//!
//!     use oxidize::dsl::*;
//!
//!     struct Transformer { is_prime: bool };
//!     let optimus = Transformer { is_prime: true };
//!
//!     expect(optimus.is_prime).to(be_true());
//!
//!     struct Paradox;
//!     impl Paradox {
//!         fn can_fly(&self) -> bool {
//!             false
//!         }
//!     }
//!     let pigs = Paradox;
//!
//!     expect(pigs.can_fly()).to(be_false());
//!
//! ### Substrings / Containers
//!
//!     use oxidize::dsl::*;
//!
//!     expect(vec![1, 2, 3]).to(contain(2));
//!     expect("Energon Cube").to(contain("Cube"));
//!
//! ### Length
//!
//!     use oxidize::dsl::*;
//!
//!     expect(vec![1, 2, 3, 4]).is_not(empty());
//!
//! ### Options / Existence
//!
//!     use oxidize::dsl::*;
//!
//!     let mut big: Vec<u64> = vec![1337u64];
//!
//!     expect(big.pop()).is(some());
//!     expect(big.pop()).is(none());
//!
//! ### Regex
//!
//!     use oxidize::dsl::*;
//!
//!     expect("sam").to(match_regex("..."));
//!
//! ## Inspiration
//!
//! These projects were inspirational in oxidize's design (maybe they'll inspire
//! you, too!):
//!
//! * [rspec-expectations](https://github.com/rspec/rspec-expectations/)
//! * [hamcrest](http://hamcrest.org/)
//! * [assertj](https://joel-costigliola.github.io/assertj/)

extern crate regex;

use matchers::Matcher;

/// Contains all built in matchers.
pub mod matchers;

/// Contains functions wrapping creating matchers in a more pleasant syntax.
pub mod dsl;

use std::fmt::Debug;

/// Holds an expectation's left hand side (LHS). This LHS can then be matched by
/// any `Matcher`.
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

