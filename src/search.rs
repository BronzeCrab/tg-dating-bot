pub fn tf(req_word: &str, words: &[&str]) -> f32 {
    let mut count: u32 = 0;
    for word in words {
        if word.trim() == req_word.trim() {
            count += 1;
        }
    }
    count as f32 / words.len() as f32
}
