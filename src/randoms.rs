use rand::{distr::Alphanumeric, Rng};

pub fn alphanumeric_string(length: usize) -> String {
    return rand::rng()
        .sample_iter(Alphanumeric)
        .take(length)
        .map(char::from)
        .collect::<String>();
}

pub fn numeric_string(length: usize) -> String {
    // TODO da fare
    return rand::rng()
        .sample_iter(Alphanumeric)
        .take(length)
        .map(char::from)
        .collect::<String>();
}
