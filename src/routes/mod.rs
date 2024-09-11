use bytes::Bytes;
use http_body_util::{combinators::BoxBody, Empty};
use hyper::{HeaderMap, Request, Response};

use http_body_util::{BodyExt, Full};
mod index;
mod upload;

pub use index::index;
pub use upload::upload;

pub type Req = Request<hyper::body::Incoming>;
pub type Res = Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error>;
pub fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

pub fn _full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

pub fn header2json<T: serde::de::DeserializeOwned>(
    header: &HeaderMap,
) -> Result<T, serde_json::Error> {
    let str_data = format!("{:?}", header);
    serde_json::from_str(&str_data)
}

#[macro_export]
macro_rules! full {
    () => {
        _full("")
    };
    ($string: expr) => {
        _full($string)
    };
    ($($k: tt), *) => {
        _full(format!($($k, )*))
    };
}
pub(crate) use full;
