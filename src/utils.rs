use crate::db_operations::{
    get_all_other_user_ids, get_tokens_by_user_id, get_user_id_by_tg_username,
};
use crate::search::compute_tf;
use rusqlite::Connection;
use std::collections::HashMap;

pub fn split_into_tokens(description: &str) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    let mut word: String = String::new();
    for el in description.chars() {
        if el.is_alphanumeric() {
            word.push_str(&el.to_lowercase().to_string());
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

pub fn compute_tf_idf(conn: &Connection, tg_username: &str) -> HashMap<String, HashMap<u32, f32>> {
    let mut tf_idf_res: HashMap<String, HashMap<u32, f32>> = HashMap::new();

    let user_id: u32 = get_user_id_by_tg_username(&conn, tg_username).unwrap();
    let user_tokens: Vec<String> = get_tokens_by_user_id(&conn, user_id);
    let other_user_ids: Vec<u32> = get_all_other_user_ids(&conn, user_id);

    for user_token in user_tokens {
        let mut token_tfs: HashMap<u32, f32> = HashMap::new();
        let mut idf_counter: f32 = 0.0;
        for other_user_id in &other_user_ids {
            let other_user_tokens: Vec<String> = get_tokens_by_user_id(&conn, *other_user_id);
            let tf: f32 = compute_tf(&user_token, other_user_tokens);
            token_tfs.insert(*other_user_id, tf);
            // it means, that user_token is indeed in other_user_tokens, so
            // increase idf here:
            if tf > 0.0 {
                idf_counter += 1.0;
            }
        }
        if idf_counter == 0.0 {
            continue;
        }
        let token_idf: f32 = (other_user_ids.len() as f32 / idf_counter).log10();

        let mut token_tf_idfs: HashMap<u32, f32> = HashMap::new();
        for (other_user_id, tf) in &token_tfs {
            let token_tf_idf: f32 = tf * token_idf;
            token_tf_idfs.insert(*other_user_id, token_tf_idf);
        }
        tf_idf_res.insert(user_token, token_tf_idfs);
    }
    tf_idf_res
}

pub fn get_users_ratings(tf_idf_map: HashMap<String, HashMap<u32, f32>>) -> HashMap<u32, f32> {
    let mut user_rating: HashMap<u32, f32> = HashMap::new();
    let mut counters: HashMap<u32, f32> = HashMap::new();

    for (_token, token_tf_idfs) in tf_idf_map {
        for (user_id, tf_idf) in token_tf_idfs {
            let rating = user_rating.entry(user_id).or_insert(0.0);
            *rating += tf_idf;
            let counter = counters.entry(user_id).or_insert(0.0);
            *counter += 1.0;
        }
    }

    for (user_id, rating) in &mut user_rating {
        let counter = counters.get(&user_id).unwrap();
        *rating /= counter;
    }

    user_rating
}

pub fn convert_hashmap_to_vec_of_tuples(user_rating: HashMap<u32, f32>) -> Vec<(u32, f32)> {
    let mut res: Vec<(u32, f32)> = vec![];
    for (user_id, user_rating) in user_rating {
        res.push((user_id, user_rating));
    }
    res
}

pub fn sort_vec_of_tuples_by_second_elem(mut user_rating: Vec<(u32, f32)>) -> Vec<(u32, f32)> {
    user_rating.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    user_rating
}
