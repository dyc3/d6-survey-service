use std::io::Cursor;

use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use serde::Serialize;

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
