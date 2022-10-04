use crate::caesor;

pub fn decode(ciphered_text: &String, key: &String) -> String {
    let mut clear_text = String::new();
    let mut key_cycle = key
        .chars()
        .cycle();

    for character in ciphered_text.chars() {
        let k = key_cycle.next().expect("Cycle failed");
        clear_text.push(caesor::decode(&k, &character));
    }

    return clear_text;
}
