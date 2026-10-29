fn main(){
    /*
    Slices
    - similar to arrays , but their length is not known at compile time
    - slice is a two word object, the first word is a pointer to the data, the second word is the length of the slice.
    - slices can be used to borrow a section of an array and have the type signature &[T]
    */

    let array1 : [i32; 10] = [0;10];
    println!("{:?}",array1);

    let slice1 = &array1[1..=4];
    println!("{:?}",slice1);
    println!("{:?}",&array1[1..]);

    let array2 : [i32; 10] = [1,2,3,4,5,6,7,8,9,10];
    println!("{:?}",array2);
    println!("{:?}",&array2[..]);
    println!("{:?}",&array2[1..]);
    println!("{:?}",&array2[3..=7]);
    println!("{:?}",&array2[1..10]);



}