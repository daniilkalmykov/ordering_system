use trybuild::TestCases;

#[test]
fn invalid_typestate_operations() {
    let tests = TestCases::new();

    tests.compile_fail("tests/compile_fail_tests/*.rs");
}
