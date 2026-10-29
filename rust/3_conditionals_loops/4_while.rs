fn main(){
    // Example 1
    let mut count = 1;

    while count <= 10 {
        if count==3 {
            count += 1;
            continue;
        }
        else if count == 5 {
            break;
        }
        println!("Count: {}", count);
        count += 1;
    }
    // while loop cannot return a value like `loop` does
    
}