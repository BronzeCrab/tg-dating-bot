pub fn compute_tf(req_word: &str, words: &[&str]) -> f32 {
    let mut count: f32 = 0.0;
    for word in words {
        if word.trim() == req_word.trim() {
            count += 1.0;
        }
    }
    count / words.len() as f32
}

pub fn compute_idf(req_word: &str, documents: &[&[&str]]) -> f32 {
    let mut count: f32 = 0.0;
    for &document in documents {
        for word in document {
            if word.trim() == req_word.trim() {
                count += 1.0;
                break;
            }
        }
    }
    // TODO: count == 0?
    (documents.len() as f32 / count).log10()
}
