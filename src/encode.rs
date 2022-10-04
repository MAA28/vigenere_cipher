use crate::caesor;

pub fn encode(clear_text: &String, key: &String) -> String {
    let mut ciphered_text = String::new();
    let mut key_cycle = key.chars().cycle();

    for character in clear_text.chars() {
        let k = key_cycle.next().expect("Cycle failed");
        ciphered_text.push(caesor::encode(&k, &character));
    }

    return ciphered_text;
}
