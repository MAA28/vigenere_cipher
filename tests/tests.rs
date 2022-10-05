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
fn crack() {
    trycmd::TestCases::new()
        .pass("tests/cmd/crack.toml");
}
