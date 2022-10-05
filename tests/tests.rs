#[test]
fn encode() {
    trycmd::TestCases::new()
        .pass("tests/cmd/encode.toml");
}

#[test]
fn decode() {
    trycmd::TestCases::new()
        .pass("tests/cmd/decode.toml");
}

#[test]
fn crack_with_length() {
    trycmd::TestCases::new()
        .pass("tests/cmd/crack_with_length.toml");
}

#[test]
fn crack() {
    trycmd::TestCases::new()
        .pass("tests/cmd/crack.toml");
}
