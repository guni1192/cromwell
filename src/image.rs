use std::fs::File;
use std::io;

use flate2::read::GzDecoder;
use reqwest;
use serde_json::{self, Value};
use tar::Archive;

pub struct Image {
    name: String,
    tag: String,
}

impl Image {
    pub fn new(name_and_tag: String) -> Image {
        let mut n: Vec<&str> = name_and_tag.split(':').collect();
        if n.len() < 2 {
            n.push("latest");
        }
        Image {
            name: n[0].to_string(),
            tag: n[1].to_string(),
        }
    }

    pub fn tar_archive(&self, path: String) {
        let tar_gz = File::open(&path).expect("");
        let tar = GzDecoder::new(tar_gz);
        let mut ar = Archive::new(tar);
        let filename = path.replace("/tmp/", "");
        let container_path = format!(
            "/var/lib/cromwell/containers/{}",
            &filename.replace(".tar.gz", "")
        );
        std::fs::create_dir(&container_path).unwrap();

        ar.unpack(&container_path)
            .expect("Failed to unpack filename");
    }

    pub fn pull(&self) -> Result<(), reqwest::Error> {
        let auth_url = format!(
            "https://auth.docker.io/token?service=registry.docker.io&scope=repository:{}:pull",
            self.name
        );
        let res_json: String = reqwest::get(auth_url.as_str())?.text()?;
        let body: Value = serde_json::from_str(res_json.as_str()).expect("parse json failed");

        if let Value::String(token) = &body["token"] {
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
            if let Value::Array(fs_layers) = &body["fsLayers"] {
                for fs_layer in fs_layers {
                    if let Value::String(blob_sum) = &fs_layer["blobSum"] {
                        let url = format!(
                            "https://registry.hub.docker.com/v2/{}/blobs/{}",
                            self.name, blob_sum
                        );

                        let mut res = reqwest::Client::new()
                            .get(url.as_str())
                            .bearer_auth(token)
                            .send()?;
                        let out_filename =
                            format!("/tmp/{}.tar.gz", blob_sum.replace("sha256:", ""));
                        let mut out = File::create(&out_filename).expect("failed to create file");
                        io::copy(&mut res, &mut out).expect("failed to copy content");
                        self.tar_archive(out_filename);
                    }
                }
            }
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_image() {
        let image = Image::new("library/alpine".to_string());
        assert_eq!(image.name, "library/alpine".to_string());
        assert_eq!(image.tag, "latest".to_string());
    }

    #[test]
    fn test_init_image_spec_tag() {
        let image = Image::new("library/alpine:3.8".to_string());
        assert_eq!(image.name, "library/alpine".to_string());
        assert_eq!(image.tag, "3.8".to_string());
    }
}
