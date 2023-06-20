use std::process::Command;

#[test]
fn run_test_script() {
    let output = Command::new(get_bash_executable())
        .current_dir("tests")
        .arg("-c")
        .arg("./run_all_tests.sh")
        .output()
        .unwrap();

    if !output.status.success() {
        eprintln!("Status: {}", output.status);
        eprintln!("Std out: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("Std err: {}", String::from_utf8_lossy(&output.stderr));

        panic!("Test script did not exit successfully");
    }
}

fn get_bash_executable() -> String {
    if cfg!(windows) {
        let output = Command::new("where").arg("bash").output().unwrap();
        let output = String::from_utf8_lossy(&output.stdout);
        let first = output.lines().next().unwrap();
        first.into()
    } else {
        "bash".into()
    }
}
