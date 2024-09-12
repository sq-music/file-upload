use base64::Engine;
use http_body_util::{BodyStream, Full};
use hyper::{body::Incoming, header::CONTENT_TYPE, Request, Response};
use multer::Multipart;

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

pub fn upload(req: Request<Incoming>) -> Res {
    // let header: FileInfo = header2json(req.headers()).unwrap();
    // println!("{}", header.x_image);
    // let mut files = multipart::server::Multipart::from(req);
    let boundary = req
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|ct| ct.to_str().ok())
        .and_then(|ct| multer::parse_boundary(ct).ok())
        .unwrap();
    let stream = BodyStream::new(req.into_body());
    let mut multipart: Multipart<'_> = Multipart::new(stream, boundary);

    async {
        while let Some(mut field) = multipart.next_field().await.unwrap() {
            // Get the field name.
            let name = field.name();

            // Get the field's filename if provided in "Content-Disposition" header.
            let file_name = field.file_name();

            // Get the "Content-Type" header as `mime::Mime` type.
            let content_type = field.content_type();

            println!(
                "Name: {:?}, FileName: {:?}, Content-Type: {:?}",
                name, file_name, content_type
            );

            // Process the field data chunks e.g. store them in a file.
            let mut field_bytes_len = 0;
            while let Some(field_chunk) = field.chunk().await.unwrap() {
                // Do something with field chunk.
                field_bytes_len += field_chunk.len();
            }

            println!("Field Bytes Length: {:?}", field_bytes_len);
        }
    };
    let base64_model = base64::engine::general_purpose::STANDARD_NO_PAD;
    // std::fs::write("./sus.png", base64_model.decode(&header.x_image).unwrap()).unwrap();
    Ok(Response::new(full!(":D")))
}
