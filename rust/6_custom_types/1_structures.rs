// An attribute to hide warnings for unused code/variables etc
#![allow(dead_code)]

// Classic C struct
#[derive(Debug)] // used to implement Debug trait for the struct
struct Person {
    name: String,
    age: u8, // u8 - unsigned 8 bits  - no negative numbers (0 - 255)
}


// Tuple Struct
#[derive(Debug)]
struct Pair (i32, i32);

/*
unit struct
- unit struct will always have zero fields
- A unit struct is a struct with no fields or data, defined simply as struct User;.
- It is mainly used when you need a distinct type to represent something or provide behavior, often with traits or marker types.
*/
#[derive(Debug)]
struct User;

fn main(){
    /*
    Structures - 3 types
    1. Tuple structs - named tuples
    2. classic C structs
    3. Unit structs - field-less, useful for generics
    */
    let person1 = Person {name:"Rajendra".to_string(), age:22};
    println!("{:?}", person1);
    println!("{:?}", person1.name);

    let pair1 = Pair(10,100);
    println!("{:?}", pair1);
    println!("{}", pair1.1);

    let user = User;
    println!("{:?}", user);

}