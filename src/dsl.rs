use core::fmt::Debug;

use super::Expectation;
use super::matchers::*;

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

