// strings2.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings2` or use the `hint` watch subcommand for a
// hint.

//DONE

fn main() {
    let word = String::from("green"); // Try not changing this line :)
    /*
    String类型提供了一个as_str方法，可以直接将其转换成切片
    let my_string = String::from("Hello, world");
    let my_slice: &str = my_string.as_str();
    println!("String{}", my_string);
    println!("Slice{}", my_slice);

    */
    let slice_word = word.as_str();
    if is_a_color_word(slice_word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}
