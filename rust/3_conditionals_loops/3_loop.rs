fn main(){
    /* rust have 3 loops
    - loop 
    - while
    - for
    */

    /* 
    loop 
    - It will run forever until we click 'ctrl + c' or stop using 'break' keyword.
    */

    // Example 1
    let mut count = 1;
    loop {
        println!("{}", count);
        count += 1;
        if count == 5{
            break;
        }
    }    // Here we don't need to use `;` at the end lo loop


    // Example 2 - loop with return value
    let mut count = 1;
    let a = loop {
        println!("{}", count);
        count += 1;
        if count == 5{
            break count; // here break will stop the loop and return value to a.
        }
    };   // Here we need to place `;` at the end of loop, When you save the result of a loop into a variable, you must put a semicolon (;) at the end.
    println!("a is {}", a);


}