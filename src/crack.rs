use std::{collections::HashMap, vec};
use primes::factors;

pub fn crack(ciphered_text: &String) -> String {
    let mut patterns: HashMap<String, Vec<u64>> = HashMap::new();

    let chars0 = ciphered_text.chars();
    let mut chars1 = ciphered_text.chars().cycle();
    let _ = chars1.next();
    let mut chars2 = ciphered_text.chars().cycle();
    let _ = chars2.next();
    let _ = chars2.next();

    for (i, (a, (b, c))) in chars0.zip(chars1.zip(chars2)).enumerate() {
        let mut part = String::new();
        part.push(a);
        part.push(b);
        part.push(c);

        if patterns.contains_key(&part) {
           let value = patterns.get_mut(&part).expect("Part doesn't exist");
           let difference = i as u64 - value.last().expect("No last in value");
            value.push(difference); 
        } else {
           patterns.insert(part, vec![i as u64]); 
        }
    }
    
    let mut possible_factors: HashMap<u64, u64> = HashMap::new();

    for key in patterns.keys() {
        println!("Pattern: {}", key);
        let values = patterns.get(key).expect("Wrong key");
        for value in values {
           print!("{} -> ", value);
            let factors = factors(*value);
            println!("{:?}", factors);
            for factor in factors {
                if possible_factors.contains_key(&factor) {
                    let k = possible_factors.get_mut(&factor).expect("Factor doesnt exist");
                    *k += 1;
                } else {
                   possible_factors.insert(factor, 1);
                }
            }
        }
    }
    
    let mut possible_offsets: Vec<u64> = vec![];

    for p in possible_factors.keys() {
        let value =  possible_factors.get(p).unwrap();
        if 2 * value > ciphered_text.len() as u64 {
            println!("{}: {}", p, possible_factors.get(p).unwrap());
            possible_offsets.push(*p);
        }
    }



    return String::new();
}
