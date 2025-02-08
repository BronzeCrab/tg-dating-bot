// TODO: refactor into one iteration
pub fn split_into_tokens(description: &str) -> Vec<&str> {
    let spl = description.split(" ");
    let mut res: Vec<&str> = vec![];
    for el in spl {
        if !el.is_empty() {
            res.push(el);
        }
    }
    res
}
