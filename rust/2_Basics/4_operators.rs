fn main(){
    /*
    Arithmetric Operators
    Addition (+)
    Subtraction (-)
    Multiplication (*)
    Division (/)
    Modulo (%)
    */

    let a = 9;
    let b = 2;
    let c = 9.0;
    let d = 2.0;
    println!("Arithmetric Operators");
    println!("-----------------------");
    println!("a = {}\nb = {}", a,b);
    println!("c = {}\nd = {}", c,d);
    println!("a + b = {}", a + b);
    println!("a - b = {}", a - b);
    println!("a * b = {}", a * b);
    println!("a / b = {}", a / b);
    println!("a % b = {}", a % b);
    println!("c / d = {}", c / d);
    println!("c % d = {}", c % d);
    // println!("a / c = {}", a / c); // error: cannot divide `{integer}` by `{float}`
    // println!("a * c = {}", a * c); // error: cannot multiply `{integer}` by `{float}`
    

    /* 
    Assignment operators
    =
    +=
    -=
    *=
    /=
    %=
    */
    let mut x = 10; // we need to declare a mutable variable
    println!("\nAssignment operators");
    println!("Start: {}", x);
    x += 5;
    println!("After += 5: {}", x);
    x -= 2;
    println!("After -= 2: {}", x);
    x *= 2;
    println!("After *= 2: {}", x);
    x /= 3;
    println!("After /= 3: {}", x);
    x %= 4;
    println!("After %= 4: {}", x);

    /*
    Comparison Operators
    ==
    !=
    >
    <
    >=
    <=
    */
    let a = 10;
    let b = 20;
    let c = 10.5;
    println!("\nComparison Operators");
    println!("a = {}, b = {}, c = {}", a,b,c);
    println!("----------------------");
    println!("a == b: {}", a == b);
    println!("a != b: {}", a != b);
    println!("a > b: {}", a > b);
    println!("a < b: {}", a < b);
    println!("a >= b: {}", a >= b);
    println!("a <= b: {}", a <= b);
    // println!("a == c : {}", a==c);  expected integer, found floating-point number

    /*
    Logical Operators
    && - logical and
    || - logical OR
    ! - logical NOT
    */
    let bool_a = true;
    let bool_b = false;
    println!("\nLogical Operators");
    println!("bool_a : {}, bool_b:  {}", bool_a, bool_b);
    println!("bool_a && bool_b: {}", bool_a && bool_b);
    println!("bool_a || bool_b: {}", bool_a || bool_b);
    println!("!bool_a : {}", !bool_a);

    /*
    Bitwise Operators
    & - bitwise AND
    | - bitwise OR
    ^ - bitwise XOR
    << - bitwise left shift
    >> - bitwise right shift
    */
    let bitwise_a = 10;
    let bitwise_b = 20;
    println!("Bitwise Operators");
    println!("-------------------");
    println!("bitwise_a : {}, bitwise_b:  {}", bitwise_a, bitwise_b);
    println!("bitwise_a & bitwise_b: {}", bitwise_a & bitwise_b);
    println!("bitwise_a | bitwise_b: {}", bitwise_a | bitwise_b);
    println!("bitwise_a ^ bitwise_b: {}", bitwise_a ^ bitwise_b);
    println!("bitwise_a << 1: {}", bitwise_a << 1);
    println!("bitwise_a >> 1: {}", bitwise_a >> 1);
}   