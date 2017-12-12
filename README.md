# sieve-rs

A simple, growable prime-sieve for Rust.

## Examples

All the primes (if you have the memory):

```rust
for prime in Sieve::infinite() {
    println!("prime = {}", prime);
}
```

Only primes below a certain value:

```rust
for prime in Sieve::bounded(1_000_000u64) {
    println!("prime = {}", prime);
}
```
