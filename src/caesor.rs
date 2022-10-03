pub fn caesor(a: char, b: char) -> char {
    let x = a as u8 - 97;
    let y = b as u8 - 97;

    let z = (x + y) % 26;

    return (z + 97) as char;
}
