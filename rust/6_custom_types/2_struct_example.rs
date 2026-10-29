#![allow(dead_code)]

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// We cannot directly write a function inside a struct, we either need to assign it at the time of object creation or we need to implement it
// #[derive(Debug)]
// struct Rectangle {
//     top_left: Point,
//     bottom_right: Point,
//     fn area(){
//         let length = top_left.x - bottom_right.x;
//         let width = top_left.y - bottom_right.y;
//         return length * width;
//     }
// }

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn area(&self) -> i32{
        let length = (self.top_left.x - self.bottom_right.x).abs();
        let width = (self.top_left.y - self.bottom_right.y).abs();
        return length * width;
    }
    
    fn perimeter(&self) -> i32 {
        let length = (self.top_left.x - self.bottom_right.x).abs();
        let width = (self.top_left.y - self.bottom_right.y).abs();
        return 2 * (length + width)
    }
}
fn main (){
    let rect1 = Rectangle{
        top_left: Point {
            x: -2,
            y: 3
        },
        bottom_right: Point {
            x: 4,
            y: 5
        },
    };
    println!("{:?}", rect1);
    println!("Area : {}", rect1.area());
    println!("Perimeter : {}", rect1.perimeter());
}