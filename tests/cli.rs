use assert_cmd::Command;

#[test]
//a test to invoke the cli with an subcommand 'generate'
fn testing() {
    let mut cmd = Command::cargo_bin("clt").unwrap();
    cmd.arg("generate")
        .args(["--len", "100"])
        .args(["--range", "50"])
        .args(["--noisemax", "10"]);
    cmd.assert().success();
}
