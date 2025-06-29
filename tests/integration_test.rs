#[test]
fn test_cli_version() {
    let output = std::process::Command::new("cargo")
        .args(&["run", "--bin", "tinyvm", "--", "version"])
        .output()
        .expect("Failed to run CLI");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("TinyVM CLI version"));
}