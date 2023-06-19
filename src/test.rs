use std::process::Command;

#[test]
fn run_test_script() {
    let x = Command::new("C:/Programs/Git/bin/bash.exe")
        .current_dir("tests")
        .arg("-c")
        .arg("./run_all_tests.sh")
        .output()
        .unwrap();

    eprintln!("Status: {}", x.status);
    eprintln!("Std out: {}", String::from_utf8_lossy(&x.stdout));
    eprintln!("Std err: {}", String::from_utf8_lossy(&x.stderr));

    if !x.status.success() {
        panic!("Test script did not exit successfully");
    }
}
