use std::fs::File;
use std::io;
use std::path::Path;

use flate2::read::GzDecoder;
use reqwest;
use tar::Archive;

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthInfo {
    token: String,
    access_token: String,
    expires_in: u32,
    issued_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    name: String,
    tag: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct FsLayer {
    blob_sum: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct History {
    v1_compatibility: String,
}

// TODO
// #[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
// struct Signature {
//     Header: String,
// }

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Manifest {
    architecture: String,
    fs_layers: Vec<FsLayer>,
    history: Vec<History>,
    name: String,
    schema_version: u32,
    // signatures: Vec<Signature>,
    tag: String,
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
        }
    }

    pub fn build_from_tar(&self, path: &str, dir_name: &str) -> io::Result<()> {
        let tar_gz = File::open(&path).expect("");
        let tar = GzDecoder::new(tar_gz);
        let mut ar = Archive::new(tar);

        let image_path = dir_name;

        if !Path::new(&image_path).exists() {
            std::fs::create_dir_all(&image_path)?;
        }

        for file in ar.entries().unwrap() {
            let mut file = file.unwrap();
            if file.unpack_in(&image_path).is_ok() {
                continue;
            };
        }

        Ok(())
    }

    pub fn pull(&mut self, dir_name: &str) -> Result<(), reqwest::Error> {
        let auth_url = format!(
            "https://auth.docker.io/token?service=registry.docker.io&scope=repository:{}:pull",
            self.name
        );
        let res_json: String = reqwest::get(auth_url.as_str())?.text()?;
        let auth: AuthInfo = serde_json::from_str(res_json.as_str()).expect("parse json failed");

        let manifests_url = format!(
            "https://registry.hub.docker.com/v2/{}/manifests/{}",
            self.name, self.tag
        );

        let res = reqwest::Client::new()
            .get(manifests_url.as_str())
            .bearer_auth(&auth.token)
            .send()?
            .text()?;

        let manifest: Manifest = serde_json::from_str(res.as_str()).expect("parse json failed");

        for fs_layer in manifest.fs_layers {
            self.download(&auth.token, &fs_layer, dir_name)
                .expect("download failed");
        }

        Ok(())
    }

    fn download(&self, token: &str, fs_layer: &FsLayer, dir_name: &str) -> std::io::Result<()> {
        let out_filename = format!("/tmp/{}.tar.gz", fs_layer.blob_sum.replace("sha256:", ""));

        if Path::new(out_filename.as_str()).exists() {
            self.build_from_tar(&out_filename, dir_name)
                .expect("cannnot build from tar");
            return Ok(());
        }

        let url = format!(
            "https://registry.hub.docker.com/v2/{}/blobs/{}",
            self.name, fs_layer.blob_sum
        );

        let mut res = reqwest::Client::new()
            .get(url.as_str())
            .bearer_auth(token)
            .send()
            .expect("failed to send requwest");
        let mut out = File::create(&out_filename)?;

        io::copy(&mut res, &mut out)?;
        self.build_from_tar(&out_filename, dir_name)
            .expect("cannnot build from tar");
        Ok(())
    }
}
