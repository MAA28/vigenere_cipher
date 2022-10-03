use colored::Colorize;
use std::env::args;

mod caesor;
mod crack;
mod decode;
mod encode;

fn main() {
    let mode = args()
        .nth(1)
        .expect("No mode given (encode, decode, crack, test)");
    assert!(mode == "encode" || mode == "decode" || mode == "crack" || mode == "test");
    if mode != "test" {
        let text = args().nth(2).expect("No text given");
        if mode != "crack" {
            let key = args().nth(3).expect("No key was given");
            if mode == "encode" {
                println!(
                    "Encoding \"{}\" with the key \"{}\"",
                    text.bold(),
                    key.to_string().bold()
                );
                let encoded_text = encode::encode(&text, &key);
                println!("{}", encoded_text.bold());
            } else if mode == "decode" {
                println!(
                    "Decoding \"{}\" with the key \"{}\"",
                    text.bold(),
                    key.to_string().bold()
                );
                let decoded_text = decode::decode(&text, &key);
                println!("{}", decoded_text.bold());
            }
        } else {
            println!("Cracking \"{}\"", text.bold());

            let cracked_text = crack::crack(&text);
            println!("{}", cracked_text.bold());
        }
    } else {
        let key = "abc".to_string();

        let clear_text = "zurwinterszeitalseinmaleintieferschneelagmussteeinarmerjungehinausgehenundholzaufeinemschlittenholenwieeresnunzusammengesuchtundaufgeladenhattewollteerweilersoerfrorenwarnochnichtnachhausgehensondernerstfeueranmachenundsicheinbisschenwaermendascharrteerdenschneewegundwieersodenerdbodenaufraeumtefandereinenkleinengoldenenschluesselnunglaubteerwoderschluesselwaeremuessteauchdasschlossdazuseingrubindererdeundfandeineiserneskaestchenwennderschluesselnurpasstdachteeressindgewisskostbaresachenindemkaestchenersuchteabereswarkeinschluessellochdaendlichentdeckteereinsabersokleindassmaneskaumsehenkonnteerprobierteundderschluesselpasstegluecklichdadrehteereinmalherumundnunmuessenwirwartenbiservollendsaufgeschlossenunddendeckelaufgemachthatdannwerdenwirerfahrenwasfuerwunderbaresachenindemkaestchenlagen".to_string();
        println!("Clear text: {}", clear_text);

        let ciphered_text = encode::encode(&clear_text, &key);
        println!("Ciphered text: {}", ciphered_text);

        let deciphered_text = decode::decode(&ciphered_text, &key);
        assert_eq!(clear_text, deciphered_text);
        println!("Decoding is working!");

        let cracked_text = crack::crack(&ciphered_text);
        assert_eq!(clear_text, cracked_text);
        println!("Cracking is working!");
    }
}
