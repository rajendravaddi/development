/*
Variables
- Variables are containers for storing data values (characters, numbers etc)
- In Rust, variables are immutable by default. This means that once a variable is declared, it cannot be changed.

*/
fn main() {
    // - To create a variable, use the let keyword:
    let x = 10;
    println!("x = {}",x);

    /* 
    variables in rust are immutable by default
    let y = 10;
    y = 20; -> causes an error

    to declare mutable variables we need to use `mut` keyword
    */
    let mut y = 10;
    println!("y before = {}",y);
    y = 20;
    println!("y after = {}",y);

    /* 
    shadowing
    shadowing is a feature in Rust that allows you to redeclare a variable with the same name, and the new variable will shadow the previous one. 
    */
    let x = 10;

    {
        let x = x + 1;
        println!("x = {}",x); // Here x = 11 // inner block can access outer block

    }
    let x = x * 2;
    println!("x = {}",x); // here x = 20 // outer block cannot access inner block

    /*
    Placeholder - {
    while printing variables using print macros we need to use placeholder {}
    println!("Hello {}", x);
    */

    /*
    Declare first
    It is possible to declare variable bindings first and initialize them later, but all variable bindings must be initialized before they are used: the compiler forbids use of uninitialized variable bindings, as it would lead to undefined behavior.
    */
    let variable1; // declaration
    
    {
        variable1 = 10;
    }
    
    println!("Variable1 : {}", variable1);

    /*

    let variable2; // declaration
    println!("Variable2 : {}", variable2);

    It will cause error, because we haven't initialized variable 2, compiler cannot assign a type to it.

    */

    /* 
    Freezing
    
    When data is bound by the same name immutably, it also freezes. Frozen data can’t be modified until the immutable binding goes out of scope:
    */
    let mut _mutable_integer = 7i32;

    {
        // Shadowing by immutable `_mutable_integer`
        let _mutable_integer = _mutable_integer;

        // Error! `_mutable_integer` is frozen in this scope
        // _mutable_integer = 50;
        // FIXME ^ Comment out this line

        // `_mutable_integer` goes out of scope
    }

    // Ok! `_mutable_integer` is not frozen in this scope
    _mutable_integer = 3;

}
