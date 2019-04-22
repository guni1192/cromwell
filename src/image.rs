use std::fs::File;
use std::io::{self, Error, ErrorKind};
use std::path::Path;

use flate2::read::GzDecoder;
use log::info;
use reqwest;
use serde_json::{self, Value};
use tar::Archive;

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    name: String,
    tag: String,
    fs_layers: Vec<String>,
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
            fs_layers: Vec::<String>::new(),
        }
    }

    pub fn build_from_tar(&self, dst_path: &str) -> io::Result<()> {
        for fs_layer in &self.fs_layers {
            let tar_gz = File::open(&fs_layer)?;
            let tar = GzDecoder::new(tar_gz);
            let mut archive = Archive::new(tar);

            if !Path::new(dst_path).exists() {
                info!("mkdir {}", dst_path);
                std::fs::create_dir_all(dst_path)?;
            }

            archive.unpack(dst_path)?;
            info!("archived layer {}", fs_layer);
        }

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
                    self.download(token, &fs_layer).expect("download failed");
                }
            }
            _ => eprintln!("unexpected type fsLayers"),
        }

        Ok(())
    }

    fn download(&mut self, token: &str, fs_layer: &Value) -> std::io::Result<()> {
        if let Value::String(blob_sum) = &fs_layer["blobSum"] {
            let out_filename = format!("/tmp/{}.tar.gz", blob_sum.replace("sha256:", ""));
            self.fs_layers.push(out_filename.clone());

            if Path::new(out_filename.as_str()).exists() {
                return Ok(());
            }

            let url = format!(
                "https://registry.hub.docker.com/v2/{}/blobs/{}",
                self.name, blob_sum
            );

            let mut res = reqwest::Client::new()
                .get(url.as_str())
                .bearer_auth(token)
                .send()
                .expect("failed to send requwest");
            let mut out = File::create(&out_filename)?;

            io::copy(&mut res, &mut out)?;
        } else {
            return Err(Error::new(
                ErrorKind::Other,
                "blobSum not found from fsLayer",
            ));
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
