// In this exercise, you'll learn some of the unique advantages that iterators
// can offer.

// TODO: Complete the `capitalize_first` function.
// "hello" -> "Hello"
fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => {
            let remain_char: String = chars.collect();
            format!("{}{}", first.to_uppercase(), remain_char)
        }
    }
}

// TODO: Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    //iterate over words
    //take one single word and pass inside capitalize first function and return it
    //push one inside new vector and return it

    let mut words_vec: Vec<String> = Vec::new();

    for word in words {
        let cap_first_letter = capitalize_first(word);
        println!("{}", cap_first_letter);
        words_vec.push(cap_first_letter);
    }

    words_vec
}

// TODO: Apply the `capitalize_first` function again to a slice of string
// slices. Return a single string.
// ["hello", " ", "world"] -> "Hello World"
fn capitalize_words_string(words: &[&str]) -> String {
    // iterating over words -> capitalize first letter and store inside single variable -> and collect into string

    let mut collect_words: String = String::new();
    for word in words {
        let capitalize_first_letter = capitalize_first(word);
        collect_words.push_str(&capitalize_first_letter);
    }

    collect_words
}

fn main() {
    // You can optionally experiment here.

    fn cap_first_letter(text: &str) -> String {
        let mut chars = text.chars();
        println!("{chars:?}");
        match chars.next() {
            None => String::new(),
            Some(first) => {
                println!("{:?}", chars);
                let remain_char: String = chars.collect();
                //   println!("{remain_char}");
                // first.to_uppercase().to_string() + remain_char
                // format("{} first.to_uppercase()")
                format!("{}{}", first.to_uppercase(), remain_char)
            }
        }
    }
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
