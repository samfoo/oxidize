![travis ci](https://travis-ci.org/samfoo/oxidize.svg)

# What is oxidize?

Oxidize is a library that makes assertions in tests easier to read.

*(Oxidize is still under development and should be considered experimental)*

## Huh?

How about an example, then?

```rust
#[test]
fn test_optimus_should_be_prime() {
    expect("optimus").to(contain("prime"));
}

// panicked at 'expected "optimus" to contain "prime"'
```
