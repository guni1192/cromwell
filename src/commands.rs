use std::process::{Command, Stdio};

pub fn exec_each(commands: &[String]) -> Result<&str, &str> {
    for command in commands.iter() {
        match Command::new("sh")
            .arg("-c")
            .arg(command)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
        {
            Ok(mut child) => child.wait().expect("Could not to wait Command"),
            Err(_) => return Err(&"Could not exec Command"),
        };
    }
    Ok("all commands successfull.")
}

#[test]
fn test_exec_each() {
    let commands = ["ls".to_string(), "ip a".to_string()];
    assert!(exec_each(&commands).is_ok());
}
