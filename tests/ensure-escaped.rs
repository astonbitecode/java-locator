use java_locator::locate_jvm_dyn_library;

#[test]
fn test_javahome_can_be_escaped() {
    println!("{:?}", std::env::current_dir());
    std::env::set_var("JAVA_HOME", "tests/[funky-javahome]/nested");
    assert_eq!(
        locate_jvm_dyn_library().expect("failed to located jvm library"),
        "tests/[funky-javahome]/nested/more*nested"
    );
}
