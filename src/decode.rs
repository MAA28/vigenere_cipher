use crate::caesor::caesor;

pub fn decode(ciphered_text: &String, key: &String) -> String {
    let mut clear_text = String::new();
    let mut key_cycle = key
        .chars()
        .map(|k| (26 - (k as u8 - 97) + 97) as char)
        .cycle();

    for character in ciphered_text.chars() {
        let k = key_cycle.next().expect("Cycle failed");
        clear_text.push(caesor(k, character));
    }

    return clear_text;
}
