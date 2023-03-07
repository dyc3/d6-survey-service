use std::io::Cursor;

use rocket::http::{ContentType, Status, Header};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use serde::Serialize;

use crate::cache::Cacheable;

#[derive(Debug)]
pub struct ApiOkCacheableResource<T>(pub T);

#[rocket::async_trait]
impl<'r, T> Responder<'r, 'static> for ApiOkCacheableResource<T>
where
    T: Serialize + Cacheable,
{
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let body = serde_json::to_string(&self.0).map_err(|e| {
            error!("could not serialize response: {e:?}");
            Status::InternalServerError
        })?;

        let mut resp = Response::build();
        resp.header(ContentType::JSON);
        self.0.last_modified_header().map(|h| resp.header(Header::new("Last-Modified", h)));
        self.0.etag_header().map(|h| resp.header(Header::new("ETag", h)));
        resp.sized_body(body.len(), Cursor::new(body))
            .ok()
    }
}

#[typeshare]
#[derive(Debug, Clone, Serialize)]
pub struct ApiErrorResponse<R> {
    #[serde(skip)]
    pub status: Status,
    pub message: R,
}

#[rocket::async_trait]
impl<'r, R> Responder<'r, 'static> for ApiErrorResponse<R>
where
    R: Serialize,
{
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let body = serde_json::to_string(&self).map_err(|e| {
            error!("could not serialize api error response: {e:?}");
            Status::InternalServerError
        })?;

        Response::build()
            .status(self.status)
            .header(ContentType::JSON)
            .sized_body(body.len(), Cursor::new(body))
            .ok()
    }
}
