fn main(){
    // In rust, the type of a varaible is decided by the value we give it.
    let my_num = 5;         // integer
    let my_double = 5.99;   // float
    let my_letter = 'D';    // character
    let my_bool = true;     // boolean
    let my_text = "Hello";  // string
    
    println!("my_num = {}", my_num);
    println!("my_double = {}", my_double);
    println!("my_letter = {}", my_letter);
    println!("my_bool = {}", my_bool);
    println!("my_text = {}", my_text);

    // It is also possible to explicitly tell Rust what type a value should be
    let my_num2: i32 = 5;          // integer
    let my_double2: f64 = 5.99;    // float
    let my_letter2: char = 'D';    // character
    let my_bool2: bool = true;     // boolean
    let my_text2: &str = "Hello";  // string

    println!("my_num2 = {}", my_num2);
    println!("my_double2 = {}", my_double2);
    println!("my_letter2 = {}", my_letter2);
    println!("my_bool2 = {}", my_bool2);
    println!("my_text2 = {}", my_text2);

    /*
    Basic data types in Rust are divided into different groups
    1. Numbers - Whole numbers and decimal numbers (i32, f64)
    2. Characters - Single letters or symbols (char)
    3. Strings - Text, a sequence of characters (&str)
    4. Booleans - true or false (bool)
    */


    /*
    Constants
    - Constant variables are used to store values that never change
    - Constants must be defined with a type (eg: i32, chat etc)
    - Constants can be declared using 'const' keyword
    - It is a good practice to declare them with uppercase
    */
    const PI: f64 = 3.14159;
    println!("PI = {}", PI);

    // const BIRTHYEAR = 1980;    //error: missing type for `const` item
    const BIRTHYEAR: i32 = 1980;
    println!("BIRTHYEAR = {}", BIRTHYEAR);

}