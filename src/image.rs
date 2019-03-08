use std::fs::File;
use std::io::{self, Error, ErrorKind, Write};
use std::path::Path;

use flate2::read::GzDecoder;

use log::info;

use reqwest;

use serde_json::{self, Value};

use tar::Archive;

use super::config::Config;

#[warn(unused_imports)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
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
            config: Config::new(None),
        }
    }

    pub fn get_full_path(&self, container_id: &str) -> String {
        format!("{}/{}", self.config.image_path, container_id)
    }

    pub fn build_from_tar(&self, path: &str, container_id: &str) -> io::Result<()> {
        info!("tar unpack start {}", path);

        let tar_gz = File::open(&path).expect("");
        let tar = GzDecoder::new(tar_gz);
        let mut ar = Archive::new(tar);

        let image_path = format!("{}/{}", self.config.image_path, container_id);

        if !Path::new(&image_path).exists() {
            info!("mkdir {}", image_path);
            std::fs::create_dir(&image_path)?;
        }

        for file in ar.entries().unwrap() {
            let mut file = file.unwrap();
            if file.unpack_in(&image_path).is_ok() {
                continue;
            };
            info!("{:?} is already exist", file.path());
        }
        info!("archived layer {}", path);

        Ok(())
    }

    pub fn put_config_json(&self, container_id: &str) -> std::io::Result<()> {
        let json_str = serde_json::to_string(&self)?;
        let json_bytes = json_str.as_bytes();

        let image_path = format!("{}/{}/config.json", self.config.image_path, container_id);
        let mut file = File::create(image_path)?;
        file.write_all(json_bytes)?;

        Ok(())
    }

    pub fn pull(&mut self, container_id: &str) -> Result<(), reqwest::Error> {
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
                    self.download(token, &fs_layer, container_id)
                        .expect("download failed");
                }
            }
            _ => eprintln!("unexpected type fsLayers"),
        }

        Ok(())
    }

    fn download(&self, token: &str, fs_layer: &Value, container_id: &str) -> std::io::Result<()> {
        if let Value::String(blob_sum) = &fs_layer["blobSum"] {
            let out_filename = format!("/tmp/{}.tar.gz", blob_sum.replace("sha256:", ""));

            if Path::new(out_filename.as_str()).exists() {
                self.build_from_tar(&out_filename, container_id)
                    .expect("cannnot build from tar");
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
            self.build_from_tar(&out_filename, container_id)
                .expect("cannnot build from tar");
            self.put_config_json(container_id)
                .expect("cannnot put jsno");
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
