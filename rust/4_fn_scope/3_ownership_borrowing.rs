fn main(){
    /*
    - Rust uses "ownership" to manage memory in a safe way.
    - Every value in Rust has a variable that’s called its owner.
    - When the owner goes out of scope, the value will be deleted.
    - We can only have one owner at a time, unless we borrow the value to another variable.

    */

    // When we assign c to d, the ownership of c is moved to d. After this point, c is no longer valid and cannot be used. This is known as "move semantics" in Rust.
    let c = String::from("Hello");
    let d = c; // ownership of c is moved to d, c is no longer valid
    // println!("c = {}",c); // This will cause an error because c is no longer valid
    println!("d = {}",d);
    
    // When we assign a to b, the ownership of a is copied to b. After this point, both a and b are valid and can be used. This is known as "copy semantics" in Rust.
    // copy semantics are only applicable to types that implement the Copy trait, such as primitive types (e.g., integers, floats, booleans) and types that are composed of these primitive types.
    //  For example, the following code will work because i32 implements the Copy trait:
    let a = 10;
    let b = a;
    println!("a = {}, b = {}",a,b);


    // Clone
    // If we want to create a deep copy of a value, we can use the clone() method. This will create a new instance of the value with its own ownership. 
    // For example, the following code will work because we are cloning c before moving its ownership to d:
    let c = String::from("Hello");
    let d = c.clone(); // create a deep copy of c and assign it to d
    println!("c = {}, d = {}",c,d); // both c and d are valid

    // Borrowing
    // In Rust, we can borrow a value by using references. A reference is a pointer to a value that allows us to access it without taking ownership. We can create a reference to a value by using the & symbol. 
    // For example, the following code will work because we are borrowing c instead of moving its ownership to d:
    let c = String::from("Hello");
    let d = &c; // borrow c and assign it to d
    println!("c = {}, d = {}",c,d); // both c and d are valid

    // Mutable Borrowing
    // In Rust, we can also borrow a value mutably by using mutable references. A mutable reference allows us to modify the value it points to. We can create a mutable reference by using the &mut symbol. 
    // For example, the following code will work because we are borrowing c mutably and modifying its value through d:
    let mut c = String::from("Hello");
    let d = &mut c; // borrow c mutably and assign it to d
    d.push_str(", world!"); // modify c through d
    println!("c = {}",c); // c is valid and has been modified
    //You can only have one mutable reference to a value at a time!
}