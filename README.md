# mkprim

A macro library for creating type-safe wrappers for rust's primitive types.

## Usage

To use `mkprim` in your rust program, add the following to your `Cargo.toml`:

```toml
[dependencies.mkprim]
git = "https://github.com/arbitrary-cat/mkprim.git"
```

Then, at the crate root, import it with the `macro_use` attribute:

```rust
#[macro_use]
extern crate mkprim;
```

Once you've done this, you can create primitive-like types using the `float_type!` macro. At the
moment this is the only macro that's implemented, but I will have `int_type!` and `uint_type!`
macros available soon. For example:

```rust
float_type!(Seconds);
```

will create a type name `Seconds` which implements the `std::num::Float` trait.
