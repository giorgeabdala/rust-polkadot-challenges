#![allow(dead_code)]
#![allow(unused_imports)]

// Lifetime annotation: ensures return value lives as long as both inputs
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn first_word<'a>(s: &'a str) -> &'a str {
    s.split_ascii_whitespace().next().unwrap_or("")
}

#[derive(PartialEq)]
// Struct with lifetime parameter - holds reference to external data
struct TextAnalyzer<'a> {
    text: &'a str, // This reference must be valid for lifetime 'a
}

impl<'a> TextAnalyzer<'a> {
    fn new(text: &'a str) -> TextAnalyzer<'a> {
        TextAnalyzer {text}
    }

    fn word_count(&self) -> usize {
        let iter = self.text.split_whitespace();
        iter.count()
    }

    fn get_text(&self) -> &'a str {
        self.text
    }

}


mod tests {
    use crate::beginner::challenge_08::{first_word, longest, TextAnalyzer};

    #[test]
    fn longest_test() {
        let string_a = "A";
        let string_b = "AA";
        let longest = longest(string_a, string_b);
        assert_eq!(longest, string_b);
    }

    #[test]
    fn first_word_test() {
        let words = "first second";
        let first_word = first_word(words);
        assert_eq!(first_word, "first");
    }

    #[test]
    fn first_word_empty_string_test() {
        let words = "";
        let first = first_word(words);
        assert_eq!(first, "");
    }
    
    #[test]
    fn first_word_whitespace_string_test() {
        let words = "     ";
        assert_eq!(first_word(words), "");
    }
    #[test]
    fn word_count_test() {
        let words = "first second";
        let analyzer = TextAnalyzer::new(words);
        let count = analyzer.word_count();
        assert_eq!(count, 2);
    }

    #[test]
    fn get_text_test() {
        let words = "first second";
        let analyzer = TextAnalyzer::new(words);
        let text = analyzer.get_text();
        assert_eq!(text, words);
    }



}