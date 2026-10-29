#![allow(dead_code)]

enum Direction {
    North,
    South,
    East,
    West,
}


fn print_direction(d:Direction){
    match d {
        Direction::North => {

            println!("North");
            println!("North");
        },
        Direction::East => println!("East"),
        Direction::West => println!("West"),
        Direction::South => println!("South"),
    }
}

fn main(){
    /*
    The `enum` keyword allows the creation of a type which may be one of few different variants.
    */

    let direction = Direction::North;
    print_direction(direction);


}