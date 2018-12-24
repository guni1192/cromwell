use std::process::Command;

pub fn pacstrap(container_path: &str) {
    let mut pacstrap = Command::new("pacstrap")
        .arg("-i")
        .arg(container_path.to_string())
        .arg("base")
        .arg("base-devel")
        .arg("dnsutils")
        .arg("--noconfirm")
        .spawn()
        .expect("Please Install arch-install-scripts");
    pacstrap.wait().expect("Failed pacstrap waiting");
}

// TODO: debootstrap
