// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!

// DONE

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    /*单独的字符串是一个字符串字面量， 类型是&static str*/
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    /*从字符串切片转成string数组*/
    string("rust is fun!".to_owned());
    /*用于将一种类型转换成另外一种类型*/
    string_slice("nice weather".into());

    /*format! 宏用于格式化字符串并返回一个新的 String*/
    string(format!("Interpolation {}", "Station"));

    /*String::from 是一个用于创建 String 类型的函数。
    它接受一个字符串切片（&str）作为参数，并返回一个拥有该字符串内容的 String 对象*/
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    /*将字符串转换为小写字母，返回一个新的 String*/
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
