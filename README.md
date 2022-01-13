# dynaport-rs

A library to get available ports.

# Installation

```rust
dynaport = { git = "https://github.com/poorlydefinedbehaviour/dynaport-rs", tag = "latest" }
```

# Docs

Run `cargo doc --no-deps --open` after adding dynaport to `cargo.toml`

# Example

```rust
println!(dynaport::random_registered_port());
```

## Inspired by [go-dynaport](https://github.com/travisjeffery/go-dynaport)
