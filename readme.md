# What is oxidize

Oxidizer is a library that makes making assertions in tests easier to read.

## Huh?

How about an example, then?

```rust
#[test]
fn test_optimus_should_be_prime() {
    expect("optimus").to(contain("prime"));
}
```
