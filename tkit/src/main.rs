mod analyze;

fn main() {
    let texts = ["Hello, World!", "This is a sample sentence.", "Rust programming is fun!"];
    let word_count = analyze::count_words(&texts);
    println!("Total words: {}", word_count);
}
