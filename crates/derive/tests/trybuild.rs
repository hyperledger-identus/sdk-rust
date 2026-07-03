#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/pass_string.rs");
    t.compile_fail("tests/ui/fail_zero_fields.rs");
    t.compile_fail("tests/ui/fail_two_fields.rs");
    t.compile_fail("tests/ui/fail_named_field.rs");
    t.compile_fail("tests/ui/fail_unrecognised_inner.rs");
    t.compile_fail("tests/ui/fail_port_suffix.rs");
}
