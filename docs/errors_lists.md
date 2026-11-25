---
level: advanced
---

# Listing the errors a method returns

## Which errors a method returns

`#[errors(..)]` on a method lists the errors it actually returns, checked against the error enum when the trait is
parsed: an unknown or repeated name is a compile error pointing at that name, not the whole attribute (see
[Errors](../api/errors.md#which-errors-a-method-returns) for the full mechanics — the model, the client check, the
evolution checker). The common errors are one entry, `CommonError`: name just the cases that apply, or leave them
out for all of them:

```rust
#[errors(CommonError(Busy, NoPower, Timeout), NoSuchChannel)]
fn read(channel: u8) -> Result<Reading, AdcError>;

#[errors(CommonError)]
fn calibrate();
```

- `CommonError(Busy, NoPower, Timeout)` — only those three common errors, plus the crate's own `NoSuchChannel`.
- `CommonError` alone — any common error.
- A list without a `CommonError` entry says nothing about common errors: `#[errors(WrongAddress, Refused)]` lists
  the crate's own variants, and any common error passes the client's check.
- An item without an error type (`fn calibrate();`, a plain property) lists only common errors; its own names are
  a compile error.

### What it does

The list changes neither the type nor the wire: it is metadata in the API model, and from there:

- a `# Errors` section, one line per entry with the variant's own doc comment (its first line, without the final
  period; a common error's from `CommonError::DOCS`; `any common error` for `CommonError` alone) — on the trait's
  own rustdoc (`ww_api::api_doc`) and on the generated client's method;
- introspection: `ww api tree -t`, `ww introspect` and Python's `help()` show the same list next to the method;
- the evolution checker: a changed `#[errors(..)]` list is reported as a *compatible* change (nothing on the wire
  moves, and the device never checks it), but worded so the reviewer sees which variants a client of the old list
  would warn about;
- a check on the client: a device replying with an error outside the method's list logs a `tracing` warning
  (`'<resource>' returned <value>, which is not in its #[errors(..)]`), a common error against the list's
  `CommonError(..)` entry (`returned CommonError(Timeout)`).
  `wire_weaver_client::set_strict_error_lists(true)` turns it into a panic instead, for the whole process; the
  `tests_common` end-to-end harness turns it on, so a device that breaks its own contract fails in CI, not in the
  field.

A method without `#[errors(..)]` may return any variant of its error enum and any common error; nothing checks it,
and clients get no `# Errors` section for it.

The same list goes on the two other items that return errors, with the same meaning and outputs:

- a stream function (`#[stream(..)] fn`, API-31): the list is about the `Err` of its `Result` and the common errors
  the open can answer with (`ww_adc::AdcStream::samples`, `ww_i2c::I2cMonitor::records`);
- a `property!`: one list above the `property!`, about its error type if it has one. A property has one error type
  for reading and writing, so one list covers both (`ww_gpio::Pin::pull`, `ww_log::LogText::level`,
  `ww_user_info::UserInfo::name`).

```rust
#[errors(NoSuchChannel, CommonError(InvalidArgument, Unsupported, Busy, InUse, NoPower))]
#[stream(SampleBlock<'i>, counted)]
fn samples(config: StreamConfig<'i>) -> Result<StreamActual, AdcError>;

#[errors(TooLong, CommonError(Busy, Fault))]
property!(rw name: MaxLen<&'i str, MAX_NAME_LEN>, UserInfoError);
```

A plain `stream!`, a `sink!`, a `const` and `ww_impl!` have no error to list, `#[errors(..)]` on them is a compile
error.

### Real examples

`ww_adc::Adc::read` lists three common errors plus the crate's own `NoSuchChannel`:

```rust
#[errors(CommonError(Busy, NoPower, Timeout), NoSuchChannel)]
fn read(channel: u8) -> Result<Reading, AdcError>;
```

`ww_adc::AdcCapture::capture` lists five:

```rust
#[errors(
    CommonError(InvalidArgument, Unsupported, Busy, InUse, NoPower),
    NoSuchChannel
)]
fn capture(config: CaptureConfig<'i>) -> Result<CaptureActual, AdcError>;
```

`ww_memory::Memory::write_many` lists the crate's own variant and one common error:

```rust
#[errors(WrongAddress, CommonError(Refused))]
fn write_many(addr: u8, data: RefVec<'i, u8>) -> Result<(), MemoryError>;
```

`ww_log::LogText::level` has no error type: only common errors can be listed:

```rust
#[errors(CommonError(Unsupported))]
property!(rw level: LevelFilter);
```

## Tools

A common error reaches a tool as the protocol's error event, so no tool matches a crate or a type name; its text
comes from `CommonError`'s `Display`, its name and doc line from `CommonError::DOCS`.

- **Python**: `RemoteError.common` holds the common error (`{"InUse": {"by": "USART2"}}`, `"Busy"`) with `.value`
  `None`, and the message is its text (`In use by USART2`); an error of the API's own is in `.value` with `.common`
  `None`.
- **ww CLI**: `ww call` and the other commands print the common error's text; `ww api tree -t` shows the
  `#[errors(..)]` lists with `CommonError(..)` entries.
