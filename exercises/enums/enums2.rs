// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a hint.
// Execute `rustlings note enums2` or use the `note enums2` watch subcommand for lzz's note.



// I AM NOT DONE

#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
