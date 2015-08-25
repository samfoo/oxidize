#![feature(core)]
extern crate core;
use core::fmt::Debug;

pub struct Expectation<Lhs: Debug> {
    to: To<Lhs>
}

pub struct To<Lhs: Debug>(Lhs);

impl<Lhs: Debug> To<Lhs> {
    fn be<T: Debug>(&self, rhs: T) -> &Self where Lhs: PartialEq<T> {
        self.equal(rhs)
    }

    fn equal<T: Debug>(&self, rhs: T) -> &Self where Lhs: PartialEq<T> {
        if self.0 != rhs {
            panic!("expected {:?} to equal {:?}", self.0, rhs);
        }
        &self
    }
}

pub fn expect<T: Debug>(lhs: T) -> Expectation<T> {
    Expectation {
        to: To(lhs)
    }
}

mod test {
    #[test]
    fn test_expect_equal_int_to_pass() {
        super::expect(1).to.equal(1);
    }

    #[test]
    fn test_expect_be_int_to_pass() {
        super::expect(1).to.be(1);
    }

    #[test]
    #[should_panic(expected="expected 1 to equal 2")]
    fn test_expect_not_equal_int_to_fail() {
        super::expect(1).to.equal(2);
    }

    #[test]
    fn test_struct_cmp() {
        #[derive(PartialEq, Debug)]
        struct Foo { a: u16, b: String }

        super::expect(Foo{ a: 10u16, b: "Hello".to_string()})
            .to.equal(Foo{ a: 10u16, b: "Hello".to_string()});
    }
}
