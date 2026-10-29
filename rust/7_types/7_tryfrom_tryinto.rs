/*
TryFrom and TryInto
===================

`TryFrom` and `TryInto` are used when a type conversion can fail.

Unlike `From` and `Into`, which are used for conversions that are
expected to always succeed, `TryFrom` and `TryInto` return a `Result`.

    From    → infallible conversion → returns the converted value
    TryFrom → fallible conversion   → returns Result<value, error>

Example:
    We want to convert an i32 into an EvenNumber.

    8  → EvenNumber(8)  → succeeds
    5  → cannot become an EvenNumber → fails
*/

use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);


// TryFrom
// -------
//
// Implement `TryFrom<i32>` to define how an i32 can be
// converted into an EvenNumber.
//
// The conversion can fail, so `try_from()` returns:
//
//     Result<EvenNumber, ()>
//
// `Ok(...)` means the conversion succeeded.
// `Err(...)` means the conversion failed.

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}


fn main() {

    // -------------------------------------------------
    // TryFrom
    // -------------------------------------------------

    let result = EvenNumber::try_from(8);

    println!("TryFrom with 8: {:?}", result);
    // Output:
    // TryFrom with 8: Ok(EvenNumber(8))


    let result = EvenNumber::try_from(5);

    println!("TryFrom with 5: {:?}", result);
    // Output:
    // TryFrom with 5: Err(())


    // -------------------------------------------------
    // TryInto
    // -------------------------------------------------

    // `TryInto` is the opposite side of `TryFrom`.

    // Because we implemented:
    //
    //     TryFrom<i32> for EvenNumber
    //
    // Rust automatically provides the corresponding:
    //
    //     TryInto<EvenNumber> for i32
    //
    // So we can write:

    let result: Result<EvenNumber, ()> = 8i32.try_into();

    println!("TryInto with 8: {:?}", result);
    // Output:
    // TryInto with 8: Ok(EvenNumber(8))


    let result: Result<EvenNumber, ()> = 5i32.try_into();

    println!("TryInto with 5: {:?}", result);
    // Output:
    // TryInto with 5: Err(())
}