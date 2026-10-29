#![allow(dead_code)]

enum ArithmeticOperations {
    Add,
    Sub,
    Mul,
    Div,
}

impl ArithmeticOperations {
    fn run (&self, a: i32, b:i32) -> i32 {
        match self {
            ArithmeticOperations :: Add => a + b,
            ArithmeticOperations :: Sub => a - b,
            ArithmeticOperations :: Mul => a * b,
            ArithmeticOperations :: Div => a / b,
        }
    }
}

type AO = ArithmeticOperations;

impl AO {
    fn print (&self, a: i32, b:i32){
        match self {
            AO :: Add => println!("{}+{}", a,b),
            AO :: Sub => println!("{}-{}", a,b),
            AO :: Mul => println!("{}*{}", a,b),
            AO :: Div => println!("{}/{}", a,b),
        }
    }
}

fn main(){
    /*
    Type aliases
    Referencing an enum with an alias,
    see ArithmeticOperations enum
    */
    
    let x = AO :: Add;
    x.print(10,15);
    println!("{}",x.run(10,15));

}