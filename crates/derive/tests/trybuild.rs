#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/pass_string.rs");
    t.compile_fail("tests/ui/fail_zero_fields.rs");
    t.compile_fail("tests/ui/fail_two_fields.rs");
    t.compile_fail("tests/ui/fail_named_field.rs");
    t.compile_fail("tests/ui/fail_unrecognised_inner.rs");
    t.compile_fail("tests/ui/fail_port_suffix.rs");
    t.compile_fail("tests/ui/fail_validate_fn_without_err.rs");
    t.compile_fail("tests/ui/fail_validate_err_without_fn.rs");
    t.compile_fail("tests/ui/fail_new_on_validated.rs");
    t.compile_fail("tests/ui/fail_parse_on_bytes_validated.rs");
}
