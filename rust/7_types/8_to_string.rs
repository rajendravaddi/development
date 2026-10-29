use std::fmt;
use std::str::FromStr;

// A simple custom type.
#[derive(Debug)]
struct Circle {
    radius: i32,
}


// --------------------------------------------------
// Converting Circle → String
// --------------------------------------------------
//
// Instead of implementing ToString directly,
// Rust recommends implementing the Display trait.
//
// Once Display is implemented, Rust automatically
// provides the .to_string() method.
//
// This also allows us to use:
//     println!("{}", circle);
//

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle with radius {}", self.radius)
    }
}


// --------------------------------------------------
// Converting String → Circle
// --------------------------------------------------
//
// To allow a String/&str to be converted into our
// custom Circle type, we implement FromStr.
//
// FromStr requires the from_str() function to return:
//
//     Result<Self, Error>
//
// because parsing can fail.
//
// For example:
//     "10"     → Ok(Circle { radius: 10 })
//     "hello"  → Err(...)
//

impl FromStr for Circle {
    // The error type returned when parsing fails.
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        // Remove surrounding whitespace.
        //
        // "   10   " → "10"
        //
        // Then parse the string into an i32.
        let radius = s.trim().parse::<i32>()?;

        // If parsing succeeds, create a Circle.
        Ok(Circle { radius })
    }
}


fn main() {

    // ==================================================
    // 1. Circle → String
    // ==================================================

    let circle = Circle { radius: 6 };

    // Because Circle implements Display,
    // Rust automatically provides .to_string().
    let text = circle.to_string();

    println!("Circle: {:?}", circle);
    println!("As String: {}", text);

    // Output:
    // Circle: Circle { radius: 6 }
    // As String: Circle with radius 6


    // ==================================================
    // 2. String → Circle
    // ==================================================

    let text = "10";

    // parse() uses the FromStr implementation.
    //
    // The type annotation `: Circle` tells Rust
    // that we want to parse the string into a Circle.
    let circle: Circle = text.parse().unwrap();

    println!("Parsed Circle: {:?}", circle);

    // Output:
    // Parsed Circle: Circle { radius: 10 }


    // ==================================================
    // 3. Parsing can fail
    // ==================================================

    let text = "hello";

    // Here "hello" cannot be converted into an i32,
    // so parsing the Circle fails.
    //
    // We don't use unwrap() here because we want to
    // see the Result instead of causing a panic.
    let result = text.parse::<Circle>();

    println!("Invalid input: {:?}", result);

    // Output:
    // Invalid input: Err(ParseIntError { kind: InvalidDigit })
}