---
level: basic
---

# ww_error

Errors every API can return (STD-16): `Unsupported`, `InUse`, `Busy`, ... look the same in every crate, generic host
code handles them without knowing the crate, and tools show them without knowing the crate either. A common error is
not part of any API's error enum: a handler returns it beside the API's own error, the protocol carries it as an
error event, and the client gets it as `CallError::Common` (see [Errors](../api/errors.md#common-errors)).

`ww_api` re-exports the crate: `ww_api::prelude` (and `wire_weaver::prelude`, which includes it) has `CommonError`
and `CommonErrorOwned` (with `std`), so an API crate needs no dependency of its own.

## `CommonError`

| Variant | Message (its doc line) | When |
|---|---|---|
| `Unsupported` | Not supported by this device (a mode, a range, a feature of the call). | The device knows the call, and can't do what was asked. |
| `InvalidArgument { arg: ww_si::Index<ww_si::units::Argument, u8>, reason }` | Argument number {arg} is invalid: {reason}. | `arg` counts from 0. |
| `InUse { by }` | In use by {by}. | A resource another function holds (`"USART2"`). |
| `Busy` | Busy with another operation, try again later. | |
| `WrongState { reason }` | Not possible in the current state: {reason}. | |
| `NoPower { rail }` | No power on {rail}. | A missing supply or IO level (`"VIO"`). |
| `Refused { reason }` | Refused: {reason}. | Possible, but the device won't (locked, a policy). |
| `Timeout` | Timed out. | |
| `Fault { code: u32 }` | Hardware fault {code:#x}. | Device-specific code. |

Strings are bounded (`reason` 64 bytes, `by` 32, `rail` 16) and may be empty: a device that saves flash sends `""`,
and the message leaves the part out (`Refused`). `CommonError` is an `Unsized` enum, so it ends in `Unknown(u32)`
(SW-47): a variant added in a newer `ww_error` reaches an older host as `Unknown(n)`, not as a decode error.

`Unsupported` is not the framework's `Unimplemented`: that one means the device has no implementation at all (a
handler returning `Unimplemented`, an item left out), and stays a framework error.

`Display` and `core::error::Error` are implemented for both types, the message is the variant's doc line with its
fields filled in, without the final period: `In use by USART2`. `CommonError::DOCS` holds the names and doc lines
in discriminant order, and `name()` gives a value's variant name: tools print a common error from it without the
crate's definition.
