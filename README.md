# dynaport-rs

A library to get available ports.

# Installation

```toml
[dependencies]
dynaport = { git = "https://github.com/poorlydefinedbehaviour/dynaport-rs", tag = "latest" }
```

# Docs

Run `cargo doc --no-deps --open` after adding dynaport to `Cargo.toml`

# Example

```rust
println!(dynaport::random_registered_port());
```

## Inspired by [go-dynaport](https://github.com/travisjeffery/go-dynaport)
