#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub image_path: String,
}

impl Config {
    pub fn new() -> Self {
        Config {
            image_path: "/var/lib/cromwell/containers".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_config() {
        let config = Config::new();
        assert_eq!(
            config.image_path,
            "/var/lib/cromwell/containers".to_string()
        )
    }
}
