use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ApiErrorResponse<R> {
    pub message: R,
}