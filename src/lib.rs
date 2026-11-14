pub trait Carry<Context, From, To>: Sized {
    type Error;

    fn carry(context: &Context, from: From) -> Result<To, Self::Error>;
}

#[allow(async_fn_in_trait)]
pub trait CarryAsync<Context, From, To>: Sized {
    type Error;

    async fn carry_async(context: &Context, from: From) -> Result<To, Self::Error>;
}
