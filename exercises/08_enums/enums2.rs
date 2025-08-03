#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug)]
enum Message {
    // TODO: Define the different variants used below.
    Resize {
        height: u8,
        width: u8
    },//a struct 
    Point { x: u8, y: u8 },// a custom type having fields
    Move (Point),// a wrapper of Point
    Echo(String),// a String
    ChangeColor (u64, u64, u64),// a tuple
    Quit// a no value variant

}

impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

fn main() {
    let messages = [
        Message::Resize {
            width: 10,
            height: 30,
        },
        Message::Move(Point { x: 10, y: 15 }),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
