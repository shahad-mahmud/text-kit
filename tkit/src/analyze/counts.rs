use regex::Regex;

pub fn count_words(text_list: &[&str]) -> usize {
    let word_count = text_list.iter().fold(0, |count, &text| {
        let re = Regex::new(r"\b\w+\b").unwrap();
        let words = re.find_iter(text).count();
        count + words
    });

    word_count
}