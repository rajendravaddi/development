#![allow(dead_code)]

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    fn value(&self) -> u32 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

fn main(){

    let coin = Coin::Quarter;

    println!("coin value : {}", coin.value());

}