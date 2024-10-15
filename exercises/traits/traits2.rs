// traits2.rs
//
// Your task is to implement the trait `AppendBar` for a vector of strings. To
// implement this trait, consider for a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time, you can do this!
//
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.

// DONE

trait AppendBar {
    fn append_bar(self) -> Self;
}

// 为 Vec<String> 实现 AppendBar trait
impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {
        self.push(String::from("Bar"));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}

// 添加 main 函数
fn main() {
    // 这里可以放一段简单的代码来初始化或运行程序
    // 目前未使用，可以留空
    let vec = vec![String::from("Hello")].append_bar();
    println!("{:?}", vec);
}
