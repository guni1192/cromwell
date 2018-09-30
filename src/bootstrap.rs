use std::process::Command;

pub fn pacstrap(container_path: &str) {
    let mut pacstrap = Command::new("pacstrap")
        .arg("-i")
        .arg(format!("{}", container_path))
        .arg("base")
        .arg("--noconfirm")
        .spawn()
        .expect("Failed Bootstrap");
    pacstrap.wait().expect("Failed pacstrap waiting");
}

// TODO: debootstrap