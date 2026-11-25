---
level: basic
---

# Common errors in an API

A handler returns a common error beside its API's own error, and the generated client gives both back to the host. This page shows the declaration, the handler on the device and the call on the host. The list of common errors is on [Common errors](ww_error.md).

## Declaring the API's own errors

An API's error enum holds only what a tool or a caller should act on specifically (`Nack`, `UnsupportedSpeed` with
the range). The common errors need no variant, and an item that can only fail with a common error needs no `Result`:

```rust
use ww_api::prelude::*;

#[ww_trait]
trait Sensor {
    fn read() -> Result<u16, SensorError>;
    /// Fails only with common errors: `Busy` while a reading runs.
    #[errors(CommonError(Busy))]
    fn calibrate();
}

#[derive_shrink_wrap(borrowed, owned(feature = "std"), derive(Debug, PartialEq, Eq, Clone), ww_repr = unib32)]
pub enum SensorError {
    /// The sensor did not answer.
    Nack,
}

ww_api::api_error!(SensorError);
```

`api_error!` gives `wire_weaver::Error<'_, SensorError>` a `From<SensorError>`, one line per error enum (and one for
its `Owned` type where the crate makes one, under `#[cfg(feature = "std")]`). A method declared `-> Result<T,
CommonError>` is refused when the trait is parsed: the common errors come with every item already.

## Device side

A handler of a method returning `Result<T, E>` returns `Result<T, wire_weaver::Error<'_, E>>`:
`Error::Common(CommonError)` or `Error::Api(E)`, and `?` takes both. A handler of a method without a `Result` returns
`T`, or `Result<T, CommonError<'_>>` when it can refuse:

```rust
fn claim(&mut self) -> Result<(), CommonError<'static>> {
    if self.uart_active {
        return Err(CommonError::InUse { by: MaxLen::new_unchecked("USART2") });
    }
    Ok(())
}

fn sample(&mut self) -> Result<u16, wire_weaver::Error<'static, SensorError>> {
    self.claim()?; // a CommonError
    if !self.acked {
        Err(SensorError::Nack)?; // the API's own error, through api_error!
    }
    Ok(self.adc.read())
}

fn read(
    &mut self,
    _cx: &mut Context<'_, impl BlockingEventOut>,
) -> RpcResult<Result<u16, wire_weaver::Error<'_, SensorError>>> {
    Ready(self.sample())
}

fn calibrate(&mut self, _cx: &mut Context<'_, impl BlockingEventOut>) -> RpcResult<Result<(), CommonError<'_>>> {
    Ready(if self.reading { Err(CommonError::Busy) } else { Ok(()) })
}
```

Properties follow the same rule: `GetResult<T, wire_weaver::Error<'_, E>>` and `SetResult<wire_weaver::Error<'_,
E>>` with an error type, `GetResult<T, CommonError<'_>>` and `SetResult<CommonError<'_>>` without one. A deferred
method answers with `<method>_send_return(out, reply_to, output)`, where `output` is the same `Result`. `ww api
scaffold` prints the signatures.

On the wire the API's own error is still the `Err` of the method's `Result` in the value; a common error is an error
event of the protocol (`ww_client_server::ErrorKind::Common`) at the item's handler site, the same event for a
method, a property, a stream function or a refused claim at the hub. The handler's `Ok` costs nothing new: the
reply is written through the same path as before.

## Host side

The generated client answers a method `-> Result<T, E>` with `Result<T, CallError<E>>`: `CallError::Device(E)` for
the API's own error, `CallError::Common(CommonErrorOwned)` for a common one, and `common()` on both call error types
gives the common error without a match:

```rust
fn is_retryable<E>(e: &CallError<E>) -> bool {
    matches!(e.common(), Some(CommonErrorOwned::Busy | CommonErrorOwned::Timeout))
}
```

A method without a `Result` answers `Result<T, wire_weaver_client::Error>`, and `Error::common()` holds the common
error the device refused with. The dynamic client, Python and `ww` read the same event, no crate's definition needed.

`tests/common_error` runs these through a generated server, the generated client and the dynamic client;
`wire_weaver_py`'s `test_errors.py` reads them from Python through the hub.
