use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn random_session_token() -> String {
    let length = 64;
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let mut rng = thread_rng();
    let token: String = (0..length)
        .map(|_| *alphabet.choose(&mut rng).unwrap())
        .collect();
    token
}