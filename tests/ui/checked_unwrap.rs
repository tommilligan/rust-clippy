#![feature(tool_lints)]

#![deny(clippy::panicking_unwrap, clippy::unnecessary_unwrap)]
#![allow(clippy::if_same_then_else)]

fn main() {
    let x = Some(());
    if x.is_some() {
        x.unwrap(); // unnecessary
    } else {
        x.unwrap(); // will panic
    }
    if x.is_none() {
        x.unwrap(); // will panic
    } else {
        x.unwrap(); // unnecessary
    }
    let mut x: Result<(), ()> = Ok(());
    if x.is_ok() {
        x.unwrap(); // unnecessary
        x.unwrap_err(); // will panic
    } else {
        x.unwrap(); // will panic
        x.unwrap_err(); // unnecessary
    }
    if x.is_err() {
        x.unwrap(); // will panic
        x.unwrap_err(); // unnecessary
    } else {
        x.unwrap(); // unnecessary
        x.unwrap_err(); // will panic
    }
    if x.is_ok() {
        x = Err(());
        x.unwrap(); // not unnecessary because of mutation of x
        // it will always panic but the lint is not smart enough to see this (it only checks if conditions).
    } else {
        x = Ok(());
        x.unwrap_err(); // not unnecessary because of mutation of x
        // it will always panic but the lint is not smart enough to see this (it only checks if conditions).
    }
}

fn test_complex_conditions() {
    let x: Result<(), ()> = Ok(());
    let y: Result<(), ()> = Ok(());
    if x.is_ok() && y.is_err() {
        x.unwrap(); // unnecessary
        x.unwrap_err(); // will panic
        y.unwrap(); // will panic
        y.unwrap_err(); // unnecessary
    } else {
        // not statically determinable whether any of the following will always succeed or always fail:
        x.unwrap();
        x.unwrap_err();
        y.unwrap();
        y.unwrap_err();
    }

    if x.is_ok() || y.is_ok() {
        // not statically determinable whether any of the following will always succeed or always fail:
        x.unwrap();
        y.unwrap();
    } else {
        x.unwrap(); // will panic
        x.unwrap_err(); // unnecessary
        y.unwrap(); // will panic
        y.unwrap_err(); // unnecessary
    }
    let z: Result<(), ()> = Ok(());
    if x.is_ok() && !(y.is_ok() || z.is_err()) {
        x.unwrap(); // unnecessary
        x.unwrap_err(); // will panic
        y.unwrap(); // will panic
        y.unwrap_err(); // unnecessary
        z.unwrap(); // unnecessary
        z.unwrap_err(); // will panic
    }
    if x.is_ok() || !(y.is_ok() && z.is_err()) {
        // not statically determinable whether any of the following will always succeed or always fail:
        x.unwrap();
        y.unwrap();
        z.unwrap();
    } else {
        x.unwrap(); // will panic
        x.unwrap_err(); // unnecessary
        y.unwrap(); // unnecessary
        y.unwrap_err(); // will panic
        z.unwrap(); // will panic
        z.unwrap_err(); // unnecessary
    }
}

fn test_nested() {
    fn nested() {
        let x = Some(());
        if x.is_some() {
            x.unwrap(); // unnecessary
        } else {
            x.unwrap(); // will panic
        }
    }
}
