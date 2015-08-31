use core::fmt::Debug;
use super::Matcher;

pub struct Not<A>(pub Box<Matcher<A>>);

impl<Lhs: Debug> Matcher<Lhs> for Not<Lhs> {
    fn matches(&self, rhs: &Lhs) -> bool {
        !self.0.matches(rhs)
    }

    fn fail_msg(&self, rhs: &Lhs) -> String {
        format!("not {}", self.0.fail_msg(rhs))
    }
}

