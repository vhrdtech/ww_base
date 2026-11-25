---
level: basic
---

# Common errors

Some failures can happen on any device: a feature the device does not have, a resource that another function holds, a supply that is off, a call that took too long. WireWeaver gives these failures one shared list, `CommonError`, so they look the same in every API. Generic host code handles them without knowing your crate, and tools show them without knowing it either.

A common error is not part of your API's own error enum. A handler returns it beside the API's own error, and the protocol carries it separately. This page shows the shared list and its messages. [Common errors in an API](ww_error_in_an_api.md) shows how a handler returns them and how a host handles them.

--8<-- "docs/assets/error_paths.svg"

*A handler can return the API's own error or a common error. They travel differently, and `common()` on the host's call error gives the common one.*

## The list

`CommonError` has nine variants that a handler can return without writing an error type of its own. An API crate needs no dependency for them: `ww_api::prelude`, and `wire_weaver::prelude` that includes it, export `CommonError` and, with `std`, `CommonErrorOwned`.

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

The strings are bounded: `reason` holds at most 64 bytes, `by` 32 and `rail` 16. They may also be empty. A device that wants to save flash sends an empty string, and the message leaves that part out.

`Unsupported` is not the framework's `Unimplemented`. `Unimplemented` means that the device has no implementation at all, for example a handler that returns it or an item left out, and it stays a framework error.

## Messages

Every variant has a message, which is its doc line with the fields filled in and without the final period. `Display` and `core::error::Error` are implemented for `CommonError` and for its owned twin `CommonErrorOwned`. `name()` gives the variant's name, and `CommonError::DOCS` lists the names and doc lines in the order of the variants. A tool prints a common error from these, without the crate's definition. [`book_errors.rs:messages`] prints each variant's name and message:

```rust
--8<-- "ww_error/tests/book_errors.rs:messages"
```

```text
--8<-- "docs/generated/ww_error/messages.txt"
```

An empty string leaves its part out of the message. [`book_errors.rs:empty`] shows `Refused` with no reason:

```rust
--8<-- "ww_error/tests/book_errors.rs:empty"
```

```text
--8<-- "docs/generated/ww_error/empty.txt"
```

## On the wire

A common error is its variant's number, then its fields. [`book_errors.rs:wire`] checks the bytes of `InUse` and of `Busy`:

```rust
--8<-- "ww_error/tests/book_errors.rs:wire"
```

`InUse` is the variant number 2, the string `USART2`, and the string's length at the back, as everywhere in the wire format. `Busy` has no fields, so it is one byte.

The list may grow. `CommonError` ends in the variant `Unknown(u32)`. A variant that a later `ww_error` adds reaches an older host as `Unknown(n)`, not as a decode error. [`book_errors.rs:unknown`] reads the bytes of a variant number 12:

```rust
--8<-- "ww_error/tests/book_errors.rs:unknown"
```

## Where to go next

- [Common errors in an API](ww_error_in_an_api.md) returns them from a handler and handles them on the host.
- [Errors](../api/errors.md) covers your API's own error enum.
- [Listing the errors a method returns](errors_lists.md) declares which errors a method can return, so that tools can show them.
