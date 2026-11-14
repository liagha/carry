# carry

Context-aware conversion traits.

`Carry` converts a value from one type to another within a context, synchronously.
`CarryAsync` does the same asynchronously.

Both traits are parameterized by `Context`, `From`, and `To`, so the same source
and target types can convert differently depending on the context supplied.

## Example

```rust
use carry::Carry;

struct Converter {
    km_per_mile: f64,
}

struct Kilometers(f64);
struct Miles(f64);

impl Carry<Converter, Kilometers, Miles> for Miles {
    type Error = std::convert::Infallible;

    fn carry(context: &Converter, from: Kilometers) -> Result<Self, Self::Error> {
        Ok(Miles(from.0 / context.km_per_mile))
    }
}

let ctx = Converter { km_per_mile: 1.609 };
let miles: Miles = Miles::carry(&ctx, Kilometers(10.0)).unwrap();
assert!((miles.0 - 6.215).abs() < 0.001);
```

## Traits

- `Carry<Context, From, To>` — `fn carry(&Context, From) -> Result<To, Error>`
- `CarryAsync<Context, From, To>` — `async fn carry_async(&Context, From) -> Result<To, Error>`

`Error` is an associated type, so each implementation chooses its own error type.
No dedicated error type is imposed on consumers.

Blanket identity impls are provided: any type carries into itself with an
`Infallible` error.

## Install

```toml
[dependencies]
carry = "0.2.0"
```

[https://crates.io/crates/carry](https://crates.io/crates/carry)

## License

MIT
