use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

use flate2::read::GzDecoder;
use reqwest;
use serde_json::{self, Value};
use tar::Archive;

use super::config::Config;

#[warn(unused_imports)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    pub id: String,
    name: String,
    tag: String,
    pub config: Config,
}

impl Image {
    pub fn new(name_and_tag: &str) -> Image {
        let mut n: Vec<&str> = name_and_tag.split(':').collect();
        if n.len() < 2 {
            n.push("latest");
        }
        Image {
            name: n[0].to_string(),
            tag: n[1].to_string(),
            id: "".to_string(),
            config: Config::new(),
        }
    }

    pub fn get_full_path(&self) -> String {
        format!("{}/{}", self.config.image_path, self.id)
    }

    pub fn tar_archive(&mut self, path: &str) -> io::Result<()> {
        println!("[INFO] tar unpack start {}", path);
        let tar_gz = File::open(&path).expect("");
        let tar = GzDecoder::new(tar_gz);
        let mut ar = Archive::new(tar);

        self.id = path.replace("/tmp/", "").replace(".tar.gz", "");
        let image_path = format!("{}/{}", self.config.image_path, self.id);

        if Path::new(&image_path).exists() {
            fs::remove_dir_all(&image_path)?;
        }

        println!("[INFO] mkdir {}", image_path);
        std::fs::create_dir(&image_path)?;

        println!("[INFO] unpacking {}", image_path);
        ar.unpack(&image_path)?;

        Ok(())
    }

    pub fn put_config_json(&self) -> std::io::Result<()> {
        let json_str = serde_json::to_string(&self)?;
        let json_bytes = json_str.as_bytes();

        let image_path = format!("{}/{}/config.json", self.config.image_path, self.id);
        let mut file = File::create(image_path)?;
        file.write_all(json_bytes)?;

        Ok(())
    }

    pub fn pull(&mut self) -> Result<(), reqwest::Error> {
        let auth_url = format!(
            "https://auth.docker.io/token?service=registry.docker.io&scope=repository:{}:pull",
            self.name
        );
        let res_json: String = reqwest::get(auth_url.as_str())?.text()?;
        let body: Value = serde_json::from_str(res_json.as_str()).expect("parse json failed");

        let token = match &body["token"] {
            Value::String(t) => t,
            _ => panic!("unexpected data: body[\"token\"]"),
        };

        let manifests_url = format!(
            "https://registry.hub.docker.com/v2/{}/manifests/{}",
            self.name, self.tag
        );

        let res = reqwest::Client::new()
            .get(manifests_url.as_str())
            .bearer_auth(token)
            .send()?
            .text()?;

        let body: Value = serde_json::from_str(res.as_str()).expect("parse json failed");

        match &body["fsLayers"] {
            Value::Array(fs_layers) => {
                for fs_layer in fs_layers {
                    self.download(token.to_string(), &fs_layer)
                        .expect("failed to download")
                }
            }
            _ => eprintln!("unexpected type fsLayers"),
        }

        Ok(())
    }

    fn download(&mut self, token: String, fs_layer: &Value) -> Result<(), reqwest::Error> {
        if let Value::String(blob_sum) = &fs_layer["blobSum"] {
            let url = format!(
                "https://registry.hub.docker.com/v2/{}/blobs/{}",
                self.name, blob_sum
            );

            let mut res = reqwest::Client::new()
                .get(url.as_str())
                .bearer_auth(token)
                .send()?;
            let out_filename = format!("/tmp/{}.tar.gz", blob_sum.replace("sha256:", ""));
            let mut out = File::create(&out_filename).expect("failed to create file");
            io::copy(&mut res, &mut out).expect("failed to copy content");

            self.tar_archive(out_filename.as_str())
                .expect("failed to un archive tar.gz");

            self.put_config_json().expect("failed to put config json");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_image() {
        let image = Image::new("library/alpine");
        assert_eq!(image.name, "library/alpine".to_string());
        assert_eq!(image.tag, "latest".to_string());
    }

    #[test]
    fn test_init_image_spec_tag() {
        let image = Image::new("library/alpine:3.8");
        assert_eq!(image.name, "library/alpine".to_string());
        assert_eq!(image.tag, "3.8".to_string());
    }
}
