/*
function syntax
fn function_name() {
    block of statements
}

calling a function
function_name();
*/

// outside main function    
fn func2() {
        println!("This is function 2");
}

// Type of parameter must be mentioned
fn func3(param: &str){
    println!("From func3 {}",param);
}

// function with return value and type
fn square(num : i32) -> i32{
    return num*num;
}

//In Rust, we can omit the return keyword. Just write the value on the last line of the function, without a semicolon
fn cube(num : i32) -> i32{
    let result = num*num*num;
    result
}
fn main(){
    // inside main function
    fn func1() {
        println!("This is function 1");
    }
    func1();
    func2();
    func3("Hello");
    println!("Square of 5 is {}",square(5));
    println!("Cube of 5 is {}",cube(5));
}