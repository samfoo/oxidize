# What is oxidize?

Oxidizer is a library that makes making assertions in tests easier to read.
Oxidize is still under development and should be considered experimental for
the time being.

## Huh?

How about an example, then?

```rust
#[test]
fn test_optimus_should_be_prime() {
    expect("optimus".to_string()).to(contain("prime".to_string()));
}

// panicked at 'expected "optimus" to contain "prime"'
```
