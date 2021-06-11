#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/discriminant_hash_derive.rs");
}
