use abc_macros::file_words;

fn main() {
    let _ = file_words!("tests/words/nonexistent");
}
