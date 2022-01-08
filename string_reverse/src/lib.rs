use std::ops::Index;

pub fn reverse(input: &str) -> String {

    // let mut new_string = String::new();
    // let str_length = input.len();
    // for i in 0..str_length {
    //     let x = str_length - i;
    //
    //     new_string.push_str(input.index(x-1..x));
    // }
    input.chars().rev().collect()
}

#[cfg(test)]
mod tests {

    fn process_reverse_case(input: &str, expected: &str) {
        assert_eq!(&super::reverse(input), expected)
    }
    #[test]
    /// empty string
    fn test_an_empty_string() {
        process_reverse_case("", "");
    }
    #[test]
    /// a word
    fn test_a_word() {
        process_reverse_case("robot", "tobor");
    }
    #[test]
    /// a capitalized word
    fn test_a_capitalized_word() {
        process_reverse_case("Ramen", "nemaR");
    }
    #[test]
    /// a sentence with punctuation
    fn test_a_sentence_with_punctuation() {
        process_reverse_case("I'm hungry!", "!yrgnuh m'I");
    }
    #[test]
    /// a palindrome
    fn test_a_palindrome() {
        process_reverse_case("racecar", "racecar");
    }
    #[test]
    /// an even-sized word
    fn test_an_even_sized_word() {
        process_reverse_case("drawer", "reward");
    }
    #[test]
    /// wide characters
    fn test_wide_characters() {
        process_reverse_case("子猫", "猫子");
    }
    #[test]
    #[cfg(feature = "grapheme")]
    /// grapheme clusters
    fn test_grapheme_clusters() {
        process_reverse_case("uüu", "uüu");
    }
}
