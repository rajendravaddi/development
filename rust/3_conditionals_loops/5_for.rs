fn main(){
    // Example 1 - looping over a range
    // 1..6 => 1,2,3,4,5
    // 1..=6 => 1,2,3,4,5,6
    for i in 1..6 {
        println!("i is: {}", i);
    }

    // Example 2 - Looping over an array
    let arr = [10, 20, 30, 40, 50];

    for i in arr {
        println!("i is: {}", i);
    }
       
}