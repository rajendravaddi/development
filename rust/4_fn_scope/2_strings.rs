fn main() {
    /*
    There are two main types of strings in Rust
    - &str - called string slices, used for fixed strings
    - String - used for dynamic strings
    */

    // 1. Creating a string
    let string1 = "Hello".to_string();
    println!("String 1 : {}", string1);

    let string2 = String::from("World");
    println!("String 2 : {}", string2);

    // 2. push_str() - add a string at the end
    let mut string3 = String::from("Hello");
    string3.push_str(" world");
    println!("push_str: {}", string3);

    // 3. push() - add one character at the end
    string3.push('!');
    println!("push: {}", string3);

    // 4. Combining strings
    let string4 = String::from("Hi");
    let string5 = String::from("Hello");
    let string6 = String::from("World");
    let string7 = format!("{}, {} {}", string4, string5, string6);
    println!("format!: {}", string7);

    let s1 = String::from("Hello");
    let s2 = String::from("World!");
    let s3 = String::from("What a beautiful day!");
    let result = s1 + " " + &s2 + " " + &s3;
    println!("+ operator: {}", result);
    // You can only add a &str to a String with +.

    // 5. String length and capacity
    let string8 = String::from("Hello");
    println!("Length of string 8 : {}", string8.len());
    println!("Capacity of string 8 : {}", string8.capacity());

    // 6. Empty check
    let string10 = String::from("");
    println!("Is string 10 empty? : {}", string10.is_empty());

    // 7. contains() - check if a string contains a substring
    let string11 = String::from("Hello");
    println!("Does string 11 contain 'el'? : {}", string11.contains("el"));

    // 8. replace() and replacen()
    let original = String::from("Rust is fast. Rust is safe.");
    let replaced = original.replace("Rust", "C++");
    let replaced_once = original.replacen("Rust", "Rustacean", 1);
    println!("replace: {}", replaced);
    println!("replacen: {}", replaced_once);

    // 9. split() and split_whitespace()
    let string14 = String::from("Hello, World!");
    let string15: Vec<&str> = string14.split(",").collect();
    println!("split(','): {:?}", string15);

    let sentence = "Rust is fun and easy";
    let words: Vec<&str> = sentence.split_whitespace().collect();
    println!("split_whitespace(): {:?}", words);

    // 10. trim(), trim_start(), trim_end()
    let padded = String::from("  Hello, Rust!  ");
    println!("trim(): '{}'", padded.trim());
    println!("trim_start(): '{}'", padded.trim_start());
    println!("trim_end(): '{}'", padded.trim_end());

    // 11. starts_with() / ends_with()
    let name = "hello.rs";
    println!("starts_with('hello')? {}", name.starts_with("hello"));
    println!("ends_with('.rs')? {}", name.ends_with(".rs"));

    // 12. to_uppercase() / to_lowercase()
    let text = "Rust";
    println!("to_uppercase(): {}", text.to_uppercase());
    println!("to_lowercase(): {}", text.to_lowercase());

    // 13. chars() and bytes()
    let sample = "hello";
    println!("chars(): {:?}", sample.chars().collect::<Vec<char>>());
    println!("bytes(): {:?}", sample.bytes().collect::<Vec<u8>>());

    // 14. lines()
    let poem = "Rust\nPython\nGo";
    println!("lines(): {:?}", poem.lines().collect::<Vec<&str>>());

    // 15. find() and rfind()
    let greeting = "Hello, world!";
    println!("find('world'): {:?}", greeting.find("world"));
    println!("rfind('o'): {:?}", greeting.rfind('o'));

    // 16. insert(), insert_str(), remove(), replace_range()
    let mut word = String::from("Hello");
    word.insert(5, '!');
    println!("insert: {}", word);

    word.insert_str(6, " there");
    println!("insert_str: {}", word);

    word.remove(0);
    println!("remove: {}", word);

    let mut message = String::from("Hello Rust");
    message.replace_range(6..10, "World");
    println!("replace_range: {}", message);

    // 17. String indexing
    let string16 = String::from("Hello");
    let string17 = &string16[0..2];
    println!("String indexing: {}", string17);

    // 18. String slicing on UTF-8 boundaries must be valid.
    let utf8_string = String::from("こんにちは");
    let part = &utf8_string[0..3];
    println!("UTF-8 slicing: {}", part);
}
