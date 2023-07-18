use assert_cmd::Command;

#[test]
fn test1() {
    let mut cmd = Command::cargo_bin("temp_to_celsius").unwrap();
    cmd.arg("5").assert().success().stdout("5° Celsius is equal to 41° Fahrenheit");
}

#[test]
fn test2() {
    let mut cmd = Command::cargo_bin("temp_to_celsius").unwrap();
    cmd.arg("95").assert().success().stdout("95° Celsius is equal to 203° Fahrenheit");
}
