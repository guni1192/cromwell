use std::fs;
use std::path::Path;

use dirs::home_dir;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub base_dir: String,
    pub image_path: String,
    pub container_path: String,
}

impl Config {
    pub fn new(path: Option<&str>) -> Self {
        let home = home_dir().unwrap();
        let home = home.to_str().expect("Could not PathBuf to str");
        let base_dir = format!("{}/.cromwell", home);
        let default_path = format!("{}/.cromwell/containers", home);

        let path_str = path.unwrap_or(&default_path);

        let path = Path::new(&path_str);

        if !path.exists() {
            fs::create_dir_all(path).expect("could not image path");
        }

        Config {
            base_dir,
            image_path: path_str.to_string(),
            container_path: path_str.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_config() {
        let config = Config::new(None);
        let home = home_dir().unwrap();
        let home = home.to_str().expect("Could not PathBuf to str");

        assert_eq!(config.image_path, format!("{}/.cromwell/containers", home))
    }
}
