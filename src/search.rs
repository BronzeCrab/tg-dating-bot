pub fn compute_tf(req_token: &String, tokens: Vec<String>) -> f32 {
    let mut count: f32 = 0.0;
    for word in &tokens {
        if word.trim().to_lowercase() == req_token.trim().to_lowercase() {
            count += 1.0;
        }
    }
    count / tokens.len() as f32
}

pub fn compute_idf(req_word: &str, documents: &[&[&str]]) -> f32 {
    let mut count: f32 = 0.0;
    for &document in documents {
        for word in document {
            if word.trim().to_lowercase() == req_word.trim().to_lowercase() {
                count += 1.0;
                break;
            }
        }
    }
    // TODO: count == 0?
    (documents.len() as f32 / count).log10()
}
