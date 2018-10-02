use std::env;
use std::fs;

fn get_container_path() -> Result<String, env::VarError> {
    let ace_container_path = "ACE_CONTAINER_PATH";
    env::var(ace_container_path)
}

pub fn delete(ctn_name: &str) -> std::io::Result<()> {
    let ctn_path = get_container_path().expect("Could not get env ACE_CONTAINER_PATH");
    let ctn_full_path = format!("{}/{}", ctn_path, ctn_name);
    fs::remove_dir_all(ctn_full_path)
}
