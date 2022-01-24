#[test]
fn ui() {
    let path = match version_check::is_feature_flaggable() {
        Some(true) => "ui-fail-nightly",
        _ => {
            if version_check::is_min_version("1.56.0").unwrap() {
                "ui-fail-stable"
            } else {
                "ui-fail-msrv"
            }
        }
    };

    let t = trybuild::TestCases::new();
    t.compile_fail(format!("tests/{}/*.rs", path));
}
