use std::env;
use std::fs;
use std::path::Path;

use super::network::Network;

pub struct Container {
    pub name: String,
    pub path: String,
}

impl Container {
    pub fn new(name: String) -> Container {
        Container {
            name: name.clone(),
            path: format!("{}/{}", get_containers_path().unwrap(), name.clone()),
        }
    }
}

pub fn get_containers_path() -> Result<String, env::VarError> {
    let ace_container_env = "ACE_CONTAINER_PATH";
    env::var(ace_container_env)
}

pub fn delete(ctn_name: &str) -> std::io::Result<()> {
    let ctn_path = get_containers_path().expect("Could not get env ACE_CONTAINER_PATH");
    let ctn_full_path = format!("{}/{}", ctn_path, ctn_name);
    fs::remove_dir_all(ctn_full_path)
}

#[test]
fn test_get_container_path() {
    let ace_container_env = "ACE_CONTAINER_PATH";
    let ace_container_path = "/var/tmp/ace-containers";
    env::set_var(ace_container_env, ace_container_path);

    assert_eq!(ace_container_path, get_container_path().unwrap())
}
