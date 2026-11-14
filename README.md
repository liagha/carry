# carry

Context-aware conversion traits.

`Carry` converts a value from one type to another within a context, synchronously.
`CarryAsync` does the same asynchronously.

Both traits are parameterized by `Context`, `From`, and `To`, so the same source
and target types can convert differently depending on the context supplied.

## Example

```rust
use carry::{Carry, CarryAsync};

struct Exchange; // context holds live rates

struct Rial(i64);
struct Dollar(f64);

impl Carry<Exchange, Dollar, Rial> for Rial {
    type Error = std::convert::Infallible;

    fn carry(_context: &Exchange, from: Dollar) -> Result<Self, Self::Error> {
        Ok(Rial((from.0 * 42_000.0).round() as i64))
    }
}

let ctx = Exchange;
let rial: Rial = Rial::carry(&ctx, Dollar(1.0)).unwrap();
assert_eq!(rial.0, 42_000);
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
