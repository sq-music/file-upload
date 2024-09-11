use hyper::Response;

use super::{_full, full, header2json, Req, Res};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct FileInfo {
    user_agent: String,
    // x_file: String,
    x_image: String,
    // x_name: String,
    // x_user: String,
}

pub fn index(req: Req) -> Res {
    let file = std::fs::read_to_string("index.html").unwrap();
    Ok(Response::new(full!("{}", file)))
}
