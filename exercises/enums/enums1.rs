// enums1.rs
//
// No hints this time! ;)

// DONE

#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit,
    Echo(String),
    Move{x: i32, y: i32},
    ChangeColor(i32,i32,i32),
}

fn main() {

    let quit_message = Message::Quit;
    let echo_message = Message::Echo(String::from("hello, world"));
    let move_message = Message::Move{x: 10, y: 20};
    let change_color_message = Message::ChangeColor(233, 4,4);
    //打印消息
    println!("{:?}", quit_message);
    println!("{:?}", echo_message);
    println!("{:?}", move_message);
    println!("{:?}", change_color_message);
}
