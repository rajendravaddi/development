fn main(){
    /*
    Array
    - Array is a collection of objects of the same type T, stored in contiguous memory
    - created using [] and their length

    */

    let array1: [i32; 5] = [1,2,3,4,5];
    println!("0 index : {}", array1[0]);
    //println!("0 index : {}", array1.get(0)); // this will raise error because .get() will not return i32 , it will return Option<&i32> implements Debug, which cannot be printed using {} and printed using {:?}
    println!("0 index : {:?}", array1.get(0)); // 0 index : Some(1)
    println!("0 index : {}", array1.get(0).unwrap());

    // array1[10] // 
    // println!("10 index : {}", array1[10]); // index out of bound error
    // println!("10 index : {:?}", array1.get(10)); // None - no error

    println!("length : {}", array1.len());

    let empty_array : [i32; 0] = [];
    println!("{}", empty_array.len());

}