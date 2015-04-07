# mkprim 
A macro library for creating type-safe wrappers for rust's primitive types.

## Usage

To use `mkprim` in your rust program, add the following to your `Cargo.toml`:

```toml
[dependencies]
num = "0.1.22"

[dependencies.mkprim]
git = "https://github.com/arbitrary-cat/mkprim.git"
```

Then, at the crate root, import the `num` crate, as well as `mkprim` with the `macro_use` attribute:

```rust
extern crate num;

#[macro_use]
extern crate mkprim;
```

Once you've done this, you can create primitive-like types using the `mkprim!` macro. 

```rust
mkprim! {
    float Kilograms(f32);
}
```

will create a type name `Kilograms` which implements the `num::Float` trait.

Right now **integral types are not implemented**, but in the future you'll be able to create
newtypes based on the integer types which automatically implement `std::num::Int`.

You can specify access rights just like you would for any other newtype-style struct. So to make the
type and its implementation public, simply do:

```rust
mkprim! {
    /// An obscure unit used primarily in examples.
    pub float Furlongs(pub f64);
}
```

You may have multiple types declared in the same `mkprim!` block:

```rust
mkprim! {
    pub float Years(f32);
    pub float Months(f32);
    pub float Days(f32);
    pub float Hours(f32);
    pub float Minutes(f32);
    pub float Seconds(f32);
}
```

[![Build Status](https://travis-ci.org/arbitrary-cat/mkprim.svg?branch=master)](https://travis-ci.org/arbitrary-cat/mkprim)
