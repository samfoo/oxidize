#![feature(core_intrinsics)]
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
//! ```rust
//! #[test]
//! fn test_optimus_should_be_prime() {
//!     expect("optimus prime").to(contain("prime"));
//! }
//! ```
//!
//! Any expectation can also be negated:
//!
//! ```rust
//! #[test]
//! fn test_optimus_should_be_prime() {
//!     expect("megatron").to_not(contain("prime"));
//! }
//! ```
//!
//! ## Built-in Matchers
//!
//! Oxidize has of built-in [matchers](matchers/index.html) that you can
//! use for all kinds of data.
//!
//! ### Equality
//!
//!     expect(1).to(equal(1));
//!     expect(6).is(greater_than(0));
//!     expect(-120).is(less_than(5));
//!
//! ### Truthiness
//!
//!     expect(optimus.is_prime).to(be_true());
//!     expect(pigs.can_fly()).to(be_false());
//!
//! ### Substrings / Containers
//!
//!     expect(vec![1, 2, 3]).to(contain(2));
//!     expect("Energon Cube").to(contain("Cube"));
//!
//! ### Length
//!
//!     expect(vec![1, 2, 3, 4]).is_not(empty());
//!
//! ### Options / Existence
//!
//!     expect(file.parse_content()).is(some());
//!     expect(response.get_header("Language")).is(none());
//!
//! ## Inspiration
//!
//! The following were inspirational in oxidize's design (maybe they'll inspire
//! you, too!):
//!
//! * [rspec-expectations](https://github.com/rspec/rspec-expectations/)
//! * [hamcrest](http://hamcrest.org/)
//! * [assertj](https://joel-costigliola.github.io/assertj/)

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

