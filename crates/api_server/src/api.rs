use std::io::Cursor;

use rocket::http::{ContentType, Header, Status};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use serde::Serialize;

use crate::cache::Cacheable;

#[derive(Debug)]
pub enum ApiOkCacheableResource<T> {
    Ok(T),
    NotModified,
}

#[rocket::async_trait]
impl<'r, T> Responder<'r, 'static> for ApiOkCacheableResource<T>
where
    T: Serialize + Cacheable,
{
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        match self {
            ApiOkCacheableResource::Ok(t) => {
                let body = serde_json::to_string(&t).map_err(|e| {
                    error!("could not serialize response: {e:?}");
                    Status::InternalServerError
                })?;

                let mut resp = Response::build();
                resp.header(ContentType::JSON);
                t.last_modified_header()
                    .map(|h| resp.header(Header::new("Last-Modified", h)));
                t.etag_header().map(|h| resp.header(Header::new("ETag", h)));
                resp.sized_body(body.len(), Cursor::new(body)).ok()
            }
            ApiOkCacheableResource::NotModified => {
                Response::build().status(Status::NotModified).ok()
            }
        }
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

#[get("/status")]
pub fn health() -> &'static str {
    "OK"
}

#[catch(default)]
pub fn default_catcher(status: Status, _req: &Request) -> ApiErrorResponse<&'static str> {
    ApiErrorResponse {
        status,
        message: match status.code {
            400 => "Bad Request",
            401 => "Unauthorized",
            404 => "Not Found",
            405 => "Method Not Allowed",
            500 => "Internal Server Error",
            _ => "Unknown Error",
        },
    }
}
