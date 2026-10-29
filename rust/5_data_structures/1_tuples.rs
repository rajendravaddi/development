fn return_tuple() -> (i32,i32, i32){
    return (1,2,3);
}
fn main(){
    /*
    Tuples
    - A tuple is a collection of different types.
    - Tuples are constructed using parantheses ()
    - each tuple itself is a value with type signature (T1, T2,...)
    - Functions can use tuples to return multiple values
    */
    let tuple1 = ('a','b','c');
    // println!("tuple1 : {}", tuple1); // error
    println!("tuple1 : {:?}", tuple1);
    println!("0 index :{}", tuple1.0);
    // println!("0 index :{}", tuple1[0]); // error

    let tuple2 = (1,2,3,4,5,6,7,8,9,10,11,12);
    println!("Tuple 2: {:?}", tuple2);

    // let tuple3 = (1,2,3,4,5,6,7,8,9,10,11,12,13);
    // println!("Tuple 2: {:?}", tuple3); // error - tuple with morethan 12 elements cannot be printed

    let tuple4 = (100,); //single element tuple
    println!("Tuple 4 : {:?}", tuple4);

    // Tuple of different type of elements
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let (a1, a2, a3) = return_tuple();
    println!("{:?}, {:?}, {:?}", a1, a2, a3);


}