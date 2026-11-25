//! The examples of the Basic pages on time (`docs/timestamps.md`, `docs/timed.md`): the code the pages include and the
//! output a reader sees, in golden files (`docs/generated/ww_si/`).
#![cfg(feature = "std")]

use shrink_wrap::prelude::*;
use std::fmt::Write as _;
use ww_si::prefix::Centi;
use ww_si::units::Kelvin;
use ww_si::{
    Fixed, Point, Second, Timed, Unwrapper,
    origins::{Celsius, DeviceMicros},
};

type Temperature = Point<Celsius, Fixed<Kelvin, Centi, i16>>;

/// A span of `seconds` in the unit a reader takes in at a glance.
fn span(seconds: f64) -> String {
    match seconds {
        s if s < 1.0 => format!("{:.1} ms", s * 1e3),
        s if s < 100.0 => format!("{s:.1} s"),
        s if s < 6_000.0 => format!("{:.1} min", s / 60.0),
        s if s < 360_000.0 => format!("{:.1} h", s / 3_600.0),
        s if s < 3.2e8 => format!("{:.1} days", s / 86_400.0),
        s => format!("{:.0} years", s / 31_557_600.0),
    }
}

#[test]
fn book_output_wrap_periods() {
    let mut out = String::new();
    writeln!(out, "{:<10}{:<10}wraps after", "storage", "step").unwrap();
    for (storage, bits) in [("u16", 16), ("u32", 32), ("u64", 64)] {
        for (step, step_s) in [("1 µs", 1e-6), ("1 ms", 1e-3)] {
            let seconds = 2f64.powi(bits) * step_s;
            writeln!(out, "{storage:<10}{step:<10}{}", span(seconds)).unwrap();
        }
    }
    book_outputs::check("ww_si/wrap_periods.txt", &out);
}

#[test]
fn a_clock_wraps_and_the_host_unwraps_it() {
    // --8<-- [start:wrap]
    // a 32-bit microsecond clock, one tick before it wraps
    let last = Second::micro(u32::MAX);
    // two ticks later it reads 1, not 4294967297
    assert_eq!(
        last.wrapping_add(&Second::micro(2u8)),
        Some(Second::micro(1u32))
    );
    // --8<-- [end:wrap]
    // --8<-- [start:unwrapper]
    // the host read the device's clock in full once: 5000.000123 s
    let mut unwrapper = Unwrapper::new(Second::micro(5_000_000_123u64)).unwrap();
    // a stream item says 705032827 µs; its low 32 bits are the closest match to the full time
    let full = unwrapper.unwrap(Second::micro(705_032_827u32)).unwrap();
    assert_eq!(full, Second::micro(5_000_000_123u64));
    // --8<-- [end:unwrapper]
}

#[test]
fn a_reading_with_its_time() {
    // --8<-- [start:timed]
    let mut buf = [0u8; 16];
    // 25.5 °C, taken 1 s after the device booted
    let reading = Timed::new(
        DeviceMicros::from_raw(1_000_000u32),
        Temperature::from_raw(2550),
    );
    // four bytes of time, then the two bytes of the temperature
    assert_eq!(
        reading.to_ww_bytes(&mut buf).unwrap(),
        [0x40, 0x42, 0x0f, 0x00, 0xf6, 0x09]
    );
    // --8<-- [end:timed]
}

#[test]
fn book_output_timed() {
    let mut out = String::new();
    macro_rules! println {
        ($($t:tt)*) => { writeln!(out, $($t)*).unwrap() };
    }
    // --8<-- [start:timed_show]
    let reading = Timed::new(
        DeviceMicros::from_raw(1_000_000u32),
        Temperature::from_raw(2550),
    );
    println!("{}", reading.t);
    println!("{}", reading.value);
    println!("{:?}", reading.t);
    // --8<-- [end:timed_show]
    book_outputs::check("ww_si/timed.txt", &out);
}
