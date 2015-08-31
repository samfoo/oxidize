pub use self::contains::Contains;
pub use self::equality::{Equal, GreaterThan, LessThan};
pub use self::length::Empty;
pub use self::combined::{AllOf, AnyOf};
pub use self::not::Not;

pub mod contains;
pub mod equality;
pub mod length;
pub mod combined;
pub mod not;

pub trait Matcher<Lhs> {
    fn matches(&self, lhs: &Lhs) -> bool;
    fn fail_msg(&self, lhs: &Lhs) -> String;
}

