use std::process::Command;

pub fn pacstrap(container_path: &str) -> Result<&str, &str> {
    match Command::new("pacstrap")
        .arg("-i")
        .arg(format!("{}", container_path))
        .arg("base")
        .arg("--noconfirm")
        .output()
    {
        Ok(_) => Ok("Bootstrap Done"),
        Err(_) => Err("Faild Bootstrap"),
    }
}
