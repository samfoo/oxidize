# What is oxidize?

Oxidize is a library that makes assertions in tests easier to read.

*(Oxidize is still under development and should be considered experimental)*

## Huh?

How about an example, then?

```rust
#[test]
fn test_optimus_should_be_prime() {
    expect("optimus".to_string()).to(contain("prime".to_string()));
}

// panicked at 'expected "optimus" to contain "prime"'
```
