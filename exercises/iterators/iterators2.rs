// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.

// DONE

// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    /*
    input.chars() 方法获取输入字符串的字符迭代器，并将其赋值给可变变量 c。
    chars() 方法会将字符串中的每个字符作为 char 类型进行迭代
    */
    let mut c = input.chars();
    match c.next() {
        None => String::new(), //如果输入字符串没有字符（即为空字符串），则返回一个新的空字符串。
        Some(first) => first.to_uppercase().collect::<String>()+c.as_str(),
        /*
        如果有第一个字符（即 Some(first)），则：
        first.to_uppercase() 将第一个字符转换为大写。注意，这里 to_uppercase() 返回的是一个 Unicode 字符串的迭代器，因此需要进一步处理。
        使用 collect::<String>() 将这个迭代器收集成一个 String。
        + c.as_str() 将剩余的字符（即 c 中的字符，以字符串形式）连接到大写字母后面，形成最终的结果。
        */
    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // vec![]
    words.iter().map(|&word| capitalize_first(word)).collect()
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    // String::new()
    words.iter().map(|&word| capitalize_first(word)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
