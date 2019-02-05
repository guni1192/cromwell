use std::fs::File;
use std::io;

use reqwest;
use serde_json::{self, Value};

pub struct Image {
    name: String,
}

impl Image {
    pub fn new(name: String) -> Image {
        Image { name }
    }

    pub fn pull(&self) -> Result<(), reqwest::Error> {
        // TODO: pull from dockerhub
        let auth_url = format!(
            "https://auth.docker.io/token?service=registry.docker.io&scope=repository:{}:pull",
            self.name
        );
        let res_json: String = reqwest::get(auth_url.as_str())?.text()?;
        let body: Value = serde_json::from_str(res_json.as_str()).expect("parse json failed");

        if let Value::String(token) = &body["token"] {
            // println!("{:#?}", token);
            let manifests_url = format!(
                "https://registry.hub.docker.com/v2/{}/manifests/latest",
                self.name
            );
            let res = reqwest::Client::new()
                .get(manifests_url.as_str())
                .bearer_auth(token)
                .send()?
                .text()?;

            let body: Value = serde_json::from_str(res.as_str()).expect("parse json failed");
            if let Value::Array(fs_layers) = &body["fsLayers"] {
                for fs_layer in fs_layers {
                    // println!("{}", fs_layer["blobSum"]);
                    if let Value::String(blob_sum) = &fs_layer["blobSum"] {
                        let url = format!(
                            "https://registry.hub.docker.com/v2/{}/blobs/{}",
                            self.name, blob_sum
                        );

                        let mut res = reqwest::Client::new()
                            .get(url.as_str())
                            .bearer_auth(token)
                            .send()?;
                        let mut out = File::create(format!("{}.tar.gz", blob_sum))
                            .expect("failed to create file");
                        io::copy(&mut res, &mut out).expect("failed to copy content");
                    }
                }
            }
        };

        Ok(())
    }
}
