#![allow(dead_code)]
use std::convert::Into;

#[derive(Debug)]
struct Number {
    val: i32,
}

impl Into<Number> for i32{
    fn into(self) -> Number{
        Number {val: self}
    }
}

/*
Into
The Into trait is simply the reciprocal of the From trait. It defines how to convert a type into another type.

Calling into() typically requires us to specify the result type as the compiler is unable to determine this most of the time.
*/
fn main(){
    let int  = 5;

    let num: Number = int.into();
    println!("{:?}", num);
}
