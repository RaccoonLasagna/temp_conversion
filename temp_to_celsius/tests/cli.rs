use assert_cmd::Command;

#[test]
fn test1() {
    let mut cmd = Command::cargo_bin("temp_to_celsius").unwrap();
    cmd.arg("41").assert().success().stdout("41° Fahrenheit is equal to 5° Celsius");
}

#[test]
fn test2() {
    let mut cmd = Command::cargo_bin("temp_to_celsius").unwrap();
    cmd.arg("203").assert().success().stdout("203° Fahrenheit is equal to 95° Celsius");
}
