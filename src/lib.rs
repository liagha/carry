/// Context-aware conversion, synchronously.
///
/// Converts a value of type `From` into a value of type `To` using the
/// supplied `Context`. Unlike [`From`], the conversion can depend on the
/// context, and it can fail with a user-defined error type.
///
/// This is intentionally open: `Error` is an associated type, so each
/// implementation chooses its own error. No dedicated error type is
/// imposed on consumers.
pub trait Carry<Context, From, To>: Sized {
    type Error;

    /// Convert `from` into `To` using `context`.
    fn carry(context: &Context, from: From) -> Result<To, Self::Error>;
}

/// Identity conversion: any type carries into itself with an
/// [`Infallible`](std::convert::Infallible) error.
impl<Context, T> Carry<Context, T, T> for T {
    type Error = std::convert::Infallible;

    fn carry(_context: &Context, from: T) -> Result<Self, Self::Error> {
        Ok(from)
    }
}

/// Context-aware conversion, asynchronously.
///
/// Async counterpart of [`Carry`]. Converts `from` into `To` using
/// `context`, potentially performing I/O.
pub trait CarryAsync<Context, From, To>: Sized {
    type Error;

    /// Convert `from` into `To` using `context`.
    async fn carry_async(context: &Context, from: From) -> Result<To, Self::Error>;
}

/// Async identity conversion: any type carries into itself with an
/// [`Infallible`](std::convert::Infallible) error.
impl<Context, T> CarryAsync<Context, T, T> for T {
    type Error = std::convert::Infallible;

    async fn carry_async(_context: &Context, from: T) -> Result<Self, Self::Error> {
        Ok(from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Ctx;

    struct Value(u32);

    impl Carry<Ctx, u32, Value> for Value {
        type Error = std::convert::Infallible;

        fn carry(_context: &Ctx, from: u32) -> Result<Self, Self::Error> {
            Ok(Value(from))
        }
    }

    impl CarryAsync<Ctx, Value, u32> for u32 {
        type Error = std::convert::Infallible;

        async fn carry_async(_context: &Ctx, from: Value) -> Result<Self, Self::Error> {
            Ok(from.0)
        }
    }

    #[test]
    fn sync_carry() {
        let ctx = Ctx;
        let v: Value = Value::carry(&ctx, 42).unwrap();
        assert_eq!(v.0, 42);
    }

    #[test]
    fn identity_carry() {
        let ctx = Ctx;
        let v: u32 = u32::carry(&ctx, 7).unwrap();
        assert_eq!(v, 7);
    }

    #[tokio::test]
    async fn async_carry() {
        let ctx = Ctx;
        let n: u32 = u32::carry_async(&ctx, Value(99)).await.unwrap();
        assert_eq!(n, 99);
    }

    #[tokio::test]
    async fn identity_carry_async() {
        let ctx = Ctx;
        let v: Value = Value::carry_async(&ctx, Value(3)).await.unwrap();
        assert_eq!(v.0, 3);
    }
}
