// single line comment

/* multiple line 
   comment 
*/

// main() function - is the entry point of the program
fn main() {

  // Printing is handled by a series of macros defined in std::fmt

  // println! macro - is used to print something to the console
  // ! - indicates that it is a macro
  println!("Hello World!");

  /*
  Macro :
  A macro is like a function, but with an exclamation mark (!) after it.
  */


  // Output
  // println!()
  println!("Hello"); // Places newline character at the end
  print!("World!"); // Does not place newline character at the end
  print!("\n"); // Newline character

  // some examples
  println!("{one} {two} {three}", two="Hi", one="Hello", three="World");
  let num =15;
  println!("Base10 : {}", num);
  println!("Base2 : {:b}", num);
  println!("Base8 : {:o}", num);
  println!("Base16 : {:x}", num);

  println!("{number:->5}", number=num);
  println!("{number:0>5}", number=num);
  
}