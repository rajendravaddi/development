#![allow(dead_code)]


enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

fn process(message: Message) {
    match message {
        Message::Quit => {
            println!("Quitting");
        }

        Message::Move { x, y } => {
            println!("Moving to {}, {}", x, y);
        }

        Message::Write(text) => {
            println!("Message: {}", text);
        }

        Message::ChangeColor(r, g, b) => {
            println!("RGB: {}, {}, {}", r, g, b);
        }
    }
}

fn main(){
    

    let a = Message::Quit;
    process(a);
    let b = Message::Move { x: 10, y: 20 };
    process(b);

    let c = Message::Write(String::from("Hello"));
    process(c);

    let d = Message::ChangeColor(255, 0, 0);
    process(d);

}