use java_locator::locate_jvm_dyn_library;

// Windows does not support `[`, `]`, or `*` in paths so this test does not apply
#[cfg(not(target_os = "windows"))]
#[test]
fn test_javahome_can_be_escaped() {
    use std::env::temp_dir;

    let test_path = temp_dir()
        .join("[funky-javahome]")
        .join("nested")
        .join("*dir*");

    std::fs::create_dir_all(&test_path).expect("failed to create directory");
    std::fs::write(test_path.join("libjvm.so"), "stub-file").unwrap();
    std::env::set_var(
        "JAVA_HOME",
        test_path.to_str().expect("no invalid characters"),
    );
    assert_eq!(
        locate_jvm_dyn_library().expect("failed to located jvm library"),
        format!(
            "{}",
            temp_dir().join("[funky-javahome]/nested/*dir*").display()
        )
    );
}
