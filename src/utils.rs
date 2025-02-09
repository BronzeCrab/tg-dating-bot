pub fn split_into_tokens(description: &str) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    let mut word: String = String::new();
    for el in description.chars() {
        if el.is_alphabetic() {
            word.push(el);
        } else if word.len() > 0 {
            res.push(word);
            word = String::new();
        };
    }
    if word.len() > 0 {
        res.push(word);
    }
    res
}
