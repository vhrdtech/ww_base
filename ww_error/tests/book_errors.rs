//! The examples of the Basic page on common errors (`docs/ww_error.md`): the code the page includes, in marked
//! sections, and the output a reader sees, in golden files (`docs/generated/ww_error/`).

use shrink_wrap::prelude::*;
use std::fmt::Write as _;
use ww_error::CommonError;
use ww_si::Index;

#[test]
fn book_output_messages() {
    let mut out = String::new();
    macro_rules! println {
        ($($t:tt)*) => { writeln!(out, $($t)*).unwrap() };
    }
    // --8<-- [start:messages]
    let errors = [
        CommonError::Unsupported,
        CommonError::InvalidArgument {
            arg: Index::new(1),
            reason: MaxLen::new_unchecked("out of range"),
        },
        CommonError::InUse {
            by: MaxLen::new_unchecked("USART2"),
        },
        CommonError::Busy,
        CommonError::WrongState {
            reason: MaxLen::new_unchecked("the output is on"),
        },
        CommonError::NoPower {
            rail: MaxLen::new_unchecked("VIO"),
        },
        CommonError::Refused {
            reason: MaxLen::new_unchecked("locked"),
        },
        CommonError::Timeout,
        CommonError::Fault { code: 0x2a },
    ];
    for error in &errors {
        println!("{:<16}{error}", error.name());
    }
    // --8<-- [end:messages]
    book_outputs::check("ww_error/messages.txt", &out);
}

#[test]
fn book_output_empty_strings() {
    let mut out = String::new();
    macro_rules! println {
        ($($t:tt)*) => { writeln!(out, $($t)*).unwrap() };
    }
    // --8<-- [start:empty]
    // a device that saves flash sends an empty string, and the message leaves that part out
    let refused = CommonError::Refused {
        reason: MaxLen::new_unchecked(""),
    };
    println!("{refused}");
    // --8<-- [end:empty]
    book_outputs::check("ww_error/empty.txt", &out);
}

#[test]
fn common_errors_on_the_wire() {
    // --8<-- [start:wire]
    let mut buf = [0u8; 32];
    let in_use = CommonError::InUse {
        by: MaxLen::new_unchecked("USART2"),
    };
    // the variant's number (2), the string, then the string's length at the back
    assert_eq!(in_use.to_ww_bytes(&mut buf).unwrap(), b"\x20USART2\x06");
    assert_eq!(CommonError::Busy.to_ww_bytes(&mut buf).unwrap(), [0x30]);
    // --8<-- [end:wire]
}

#[test]
fn a_variant_of_a_newer_version_is_unknown() {
    // --8<-- [start:unknown]
    // a host built with this ww_error reads a variant a later ww_error added as `Unknown(n)`, not as a decode error
    let from_the_future = CommonError::from_ww_bytes(&[0xc1]).unwrap();
    assert_eq!(from_the_future, CommonError::Unknown(12));
    assert_eq!(
        from_the_future.to_string(),
        "Common error 12 from a newer version"
    );
    // --8<-- [end:unknown]
}
