# carry

Context-aware conversion traits.

`Carry` converts a value from one type to another within a context, synchronously.
`CarryAsync` does the same asynchronously.

Both traits are parameterized by `Context`, `From`, and `To`, so the same source
and target types can convert differently depending on the context supplied.
