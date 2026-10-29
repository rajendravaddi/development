/*
From and Into
- used for type conversion

From
- The From trait allows for a type to define how to create itself from another type

ex : 
let str = "Hello";
let str2 = String::from(str);
*/
#![allow(dead_code)]
use std::convert::From;

#[derive(Debug)]
struct Number {
    val: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { val: item}
    }
}
fn main(){

    let num = Number::from(30);
    println!("{:?}", num);
}