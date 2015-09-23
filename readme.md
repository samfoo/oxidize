![https://travis-ci.org/samfoo/oxidize](https://travis-ci.org/samfoo/oxidize.svg)

# Oxidize

Oxidize is a library that makes assertions in tests easier to read and write.

Oxidize lets you set expected outcomes on objects in unit tests (or anywhere).
This allows for more fluid expression than with assertions alone and yields
more readable error messages.

For example:

    use oxidize::dsl::*;

    expect("optimus prime").to(contain("prime"));

Any expectation can also be negated:

    use oxidize::dsl::*;

    expect("megatron").to_not(contain("prime"));

## Built-in Matchers

Oxidize has of built-in
[matchers](http://samfoo.github.io/oxidize/oxidize/matchers/index.html) that you can
use for all kinds of data.

### Equality

    use oxidize::dsl::*;

    expect(1).to(equal(1));
    expect(6).is(greater_than(0));
    expect(-120).is(less_than(5));

### Truthiness

    use oxidize::dsl::*;

    struct Transformer { is_prime: bool };
    let optimus = Transformer { is_prime: true };

    expect(optimus.is_prime).to(be_true());

    struct Paradox;
    impl Paradox {
        fn can_fly(&self) -> bool {
            false
        }
    }
    let pigs = Paradox;

    expect(pigs.can_fly()).to(be_false());

### Substrings / Containers

    use oxidize::dsl::*;

    expect(vec![1, 2, 3]).to(contain(2));
    expect("Energon Cube").to(contain("Cube"));

### Length

    use oxidize::dsl::*;

    expect(vec![1, 2, 3, 4]).is_not(empty());

### Options / Existence

    use oxidize::dsl::*;

    let mut big: Vec<u64> = vec![1337u64];

    expect(big.pop()).is(some());
    expect(big.pop()).is(none());

## Inspiration

These projects were inspirational in oxidize's design (maybe they'll inspire
you, too!):

* [rspec-expectations](https://github.com/rspec/rspec-expectations/)
* [hamcrest](http://hamcrest.org/)
* [assertj](https://joel-costigliola.github.io/assertj/)
