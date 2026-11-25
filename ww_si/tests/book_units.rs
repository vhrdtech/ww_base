//! The examples of the Basic pages on numbers with units: the code the pages include, in marked sections, and the
//! output a reader sees, in golden files (`docs/generated/ww_si/`, `just book-outputs` writes them again).
//!
//! Pages: `docs/ww_si.md`, `docs/exact_units.md`, `docs/bounds.md`, `docs/quantity.md` and `docs/points.md`.
//! The types are the thermometer's of the WireWeaver book (`examples/thermometer_api`).
#![cfg(feature = "std")]

use shrink_wrap::prelude::*;
use std::fmt::Write as _;
use ww_si::prefix::{Centi, One};
use ww_si::units::{self, Hertz, Kelvin, Ratio};
use ww_si::{
    Fixed, Micros, Millis, Point, Prefix, Quantity, Range, RangeDec, Rounding, Second, Volt,
    origins::Celsius,
};

// --8<-- [start:fixed_types]
/// Sampling rate: 1 Hz to 1000 Hz in steps of 1 Hz.
type Rate = Fixed<Hertz, One, u16, Range<1, 1000>>;
/// Amplifier gain: 1 to 16, no unit.
type Gain = Fixed<Ratio, One, u8, Range<1, 16>>;
/// A temperature in steps of 0.01 K, from -327.68 °C to 327.67 °C.
type Temperature = Point<Celsius, Fixed<Kelvin, Centi, i16>>;
// --8<-- [end:fixed_types]

#[test]
fn book_output_fixed_values() {
    let mut out = String::new();
    macro_rules! println {
        ($($t:tt)*) => { writeln!(out, $($t)*).unwrap() };
    }
    // --8<-- [start:fixed_values]
    let rate = Rate::try_new(50).unwrap();
    let gain = Gain::bounded(8);
    let timeout = Micros::<u32>::new(1500);
    println!("{rate}");
    println!("{gain}");
    println!("{timeout}");
    println!("{gain:?}");
    // --8<-- [end:fixed_values]
    book_outputs::check("ww_si/fixed_values.txt", &out);
}

#[test]
fn fixed_bytes_are_the_plain_number() {
    // --8<-- [start:fixed_bytes]
    let mut buf = [0u8; 8];
    let timeout = Micros::<u32>::new(1500);
    assert_eq!(
        timeout.to_ww_bytes(&mut buf).unwrap(),
        [0xdc, 0x05, 0x00, 0x00]
    );
    // the plain u32 has the same four bytes
    assert_eq!(
        1500u32.to_ww_bytes(&mut buf).unwrap(),
        [0xdc, 0x05, 0x00, 0x00]
    );
    // a Gain is one byte, a Rate two
    assert_eq!(Gain::bounded(8).to_ww_bytes(&mut buf).unwrap(), [0x08]);
    assert_eq!(
        Rate::try_new(50).unwrap().to_ww_bytes(&mut buf).unwrap(),
        [0x32, 0x00]
    );
    // --8<-- [end:fixed_bytes]
}

#[test]
fn fixed_aliases() {
    // --8<-- [start:aliases]
    let window = Millis::<u16>::new(250);
    let uptime = Micros::<u64>::new(1_730_000_000_000_000);
    // an alias is the same type under a shorter name
    let same: Fixed<units::Second, ww_si::prefix::Milli, u16> = window;
    assert_eq!(same, window);
    assert_eq!(uptime.to_string(), "1730000000000000 µs");
    // --8<-- [end:aliases]
}

#[test]
fn kinds_on_the_wire() {
    // --8<-- [start:kinds]
    let mut buf = [0u8; 8];
    // Fixed: the API fixed the step and the storage, so the value is the number alone
    let fixed = Micros::<u32>::new(1500);
    assert_eq!(
        fixed.to_ww_bytes(&mut buf).unwrap(),
        [0xdc, 0x05, 0x00, 0x00]
    );
    // Quantity: the device picks them, so the value says which
    let quantity = Second::micro(1500u32);
    assert_eq!(
        quantity.to_ww_bytes(&mut buf).unwrap(),
        [0x23, 0xdc, 0x05, 0x00, 0x00]
    );
    // --8<-- [end:kinds]
}

#[test]
fn book_output_refused() {
    let mut out = String::new();
    macro_rules! println {
        ($($t:tt)*) => { writeln!(out, $($t)*).unwrap() };
    }
    // --8<-- [start:refused]
    // a value in the source: a bad one fails the build in `const`, panics anywhere else
    const GAIN_DEFAULT: Gain = Gain::bounded(4);
    println!("{GAIN_DEFAULT}");
    // a value that arrives at run time: a bad one is an error
    println!("{:?}", Gain::try_new(8));
    println!("{:?}", Gain::try_new(20));
    // bytes that arrive from the wire are checked the same way
    println!("{:?}", Gain::from_ww_bytes(&[8]));
    println!("{:?}", Gain::from_ww_bytes(&[20]));
    // --8<-- [end:refused]
    book_outputs::check("ww_si/refused.txt", &out);
}

#[test]
fn book_output_rail() {
    let mut out = String::new();
    macro_rules! println {
        ($($t:tt)*) => { writeln!(out, $($t)*).unwrap() };
    }
    // --8<-- [start:rail]
    // a supply rail from 0 V to 3.3 V, whatever prefix a value comes with
    type Rail = Quantity<units::Volt, RangeDec<0, 33, -1>>;
    println!("{:?}", Rail::try_new(Prefix::Milli, 3300u16).is_ok());
    println!("{:?}", Rail::try_new(Prefix::Milli, 3301u16));
    println!("{:?}", Rail::try_new(Prefix::Unit, 3u8).is_ok());
    // --8<-- [end:rail]
    book_outputs::check("ww_si/rail.txt", &out);
}

#[test]
fn book_output_quantity() {
    let mut out = String::new();
    macro_rules! println {
        ($($t:tt)*) => { writeln!(out, $($t)*).unwrap() };
    }
    // --8<-- [start:quantity]
    let supply = Volt::milli(3300u16);
    println!("{supply}");
    println!("{supply:#}");
    println!("{supply:?}");
    println!("{}", supply.to_f64());
    // values compare by what they mean, not by prefix or number type
    assert_eq!(Volt::milli(1500u16), Volt::base(1.5f32));
    assert_eq!(supply.rescaled_exact(Prefix::Unit), None);
    println!("{:?}", Volt::milli(3000u16).rescaled_exact(Prefix::Unit));
    println!(
        "{:?}",
        supply.to_in::<u32>(
            Prefix::Micro,
            Rounding::NearestEven,
            ww_si::Overflow::Checked
        )
    );
    // --8<-- [end:quantity]
    book_outputs::check("ww_si/quantity.txt", &out);
}

#[test]
fn quantity_bytes() {
    // --8<-- [start:quantity_bytes]
    let mut buf = [0u8; 8];
    // the prefix and the number type share one byte, then comes the number
    let supply = Volt::milli(3300u16);
    assert_eq!(supply.to_ww_bytes(&mut buf).unwrap(), [0x12, 0xe4, 0x0c]);
    let back = Volt::from_ww_bytes(&[0x12, 0xe4, 0x0c]).unwrap();
    assert_eq!(back, supply);
    // --8<-- [end:quantity_bytes]
}

#[test]
fn book_output_points() {
    let mut out = String::new();
    macro_rules! println {
        ($($t:tt)*) => { writeln!(out, $($t)*).unwrap() };
    }
    // --8<-- [start:points]
    let morning = Temperature::from_raw(2250);
    let noon = Temperature::from_raw(2550);
    // two points give an amount: how much warmer
    let rise = noon - morning;
    println!("{morning}");
    println!("{noon}");
    println!("{rise}");
    // a point and an amount give a point
    println!("{}", morning + rise);
    println!("{}", Temperature::from_raw(-27315));
    // --8<-- [end:points]
    book_outputs::check("ww_si/points.txt", &out);
}

#[test]
fn point_bytes() {
    // --8<-- [start:point_bytes]
    let mut buf = [0u8; 8];
    // 25.5 °C is 2550 steps of 0.01 K: the two bytes of the i16 2550
    let noon = Temperature::from_raw(2550);
    assert_eq!(noon.to_ww_bytes(&mut buf).unwrap(), [0xf6, 0x09]);
    assert_eq!(2550i16.to_ww_bytes(&mut buf).unwrap(), [0xf6, 0x09]);
    // --8<-- [end:point_bytes]
}
