use std::process::Command;

pub fn pacstrap(container_path: &str) {
    let mut pacstrap = Command::new("pacstrap")
        .arg("-i")
        .arg(format!("{}", container_path))
        .arg("base")
        .arg("--noconfirm")
        .spawn()
        .expect("Faild Bootstrap");
    pacstrap.wait();
}

// TODO: debootstrap
