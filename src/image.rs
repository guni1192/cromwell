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
        let res_json: String = reqwest::get(
            "https://auth.docker.io/token?service=registry.docker.io&scope=repository:library/alpine:pull",
        )?
        .text()?;
        let body: Value = serde_json::from_str(res_json.as_str()).expect("hoge");

        if let Value::String(token) = &body["token"] {
            println!("{:#?}", token);
            let res = reqwest::Client::new()
                .get("https://registry.hub.docker.com/v2/library/alpine/manifests/latest")
                .bearer_auth(token)
                // .header(WWW_AUTHENTICATE(Authorization(Bearer { token: token })))
                .send()?
                .text()?;
            // println!("{:#?}", res.clone());
            let body: Value = serde_json::from_str(res.as_str()).expect("hoge");
            if let Value::Array(fsLayers) = &body["fsLayers"] {
                for fsLayer in fsLayers {
                    println!("{}", fsLayer["blobSum"]);
                    if let Value::String(blobSum) = &fsLayer["blobSum"] {
                        let url = format!(
                            "https://registry.hub.docker.com/v2/library/alpine/blobs/{}",
                            blobSum
                        );

                        let mut res = reqwest::Client::new()
                            .get(url.as_str())
                            .bearer_auth(token)
                            .send()
                            .expect("hoge");
                        let mut out = File::create(format!("{}.tar.gz", blobSum))
                            .expect("failed to create file");
                        io::copy(&mut res, &mut out).expect("failed to copy content");
                    }
                }
            }
        };

        Ok(())
    }
}
