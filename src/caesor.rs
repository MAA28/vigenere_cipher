pub fn encode(a: &char, b: &char) -> char {
    let x = *a as u8 - 97;
    let y = *b as u8 - 97;

    let z = (x + y) % 26;

    return (z + 97) as char;
}

pub fn decode(ciphered_text: &char, key: &char) -> char {
    let reversed_key = (26 - (*key as u8 - 97) + 97) as char;
    return encode(&ciphered_text, &reversed_key);
}


pub fn crack(ciphered_text: &String) -> char {
    println!("{}", ciphered_text);
    let avg_dist = [
        /*A*/ 0.0558, /*B*/ 0.0196, /*C*/ 0.0316, /*D*/ 0.0498,
        /*E*/ 0.1693, /*F*/ 0.0149, /*G*/ 0.0302, /*H*/ 0.0498,
        /*I*/ 0.0802, /*J*/ 0.0024, /*K*/ 0.0132, /*L*/ 0.0360,
        /*M*/ 0.0255, /*N*/ 0.1053, /*O*/ 0.0224, /*P*/ 0.0067,
        /*Q*/ 0.0002, /*R*/ 0.0689, /*S*/ 0.0642, /*T*/ 0.0579,
        /*U*/ 0.0383, /*V*/ 0.0084, /*W*/ 0.0178, /*X*/ 0.0005,
        /*Y*/ 0.0005, /*Z*/ 0.0121,
    ];

    // get dist of text
    let mut dist = [0.000; 26];
    for character in ciphered_text.chars() {
        let k = character as usize - 97;
        dist[k] += 1.0 / ciphered_text.len() as f64;
    }

    let mut best_key = 0;
    let mut min_cost = 1000.0;
    for k in 0..26 {
        let costs = avg_dist
            .into_iter()
            .enumerate()
            .map(|(i, x)| (dist[(i + k) % 26] - x) * (dist[(i + k) % 26] - x));
        let cost: f64 = costs.sum();
        

        if cost < min_cost {
            min_cost = cost;
            best_key = k;
        }
    }

    let solved_key = (best_key as u8 + 97) as char;
    return solved_key;
}
