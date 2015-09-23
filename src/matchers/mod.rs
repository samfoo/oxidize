pub use self::contains::Contains;
pub use self::equality::{Equal, GreaterThan, LessThan};
pub use self::length::Empty;
pub use self::option::{Nothing, Something};

pub mod contains;
pub mod equality;
pub mod length;
pub mod option;

pub trait Matcher<Lhs> {
    fn matches(&self, lhs: &Lhs) -> bool;
    fn fail_msg(&self, lhs: &Lhs) -> String;
    fn negated_fail_msg(&self, lhs: &Lhs) -> String;
}

