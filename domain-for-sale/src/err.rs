use axum::{response::IntoResponse, Json};

use crate::resp;

pub struct Error(anyhow::Error);

impl Error {
    pub fn msg(&self) -> String {
        self.0.to_string()
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        Json(resp::Resp::err(self)).into_response()
    }
}

impl<E> From<E> for Error
where
    E: Into<anyhow::Error>,
{
    fn from(e: E) -> Self {
        Self(e.into())
    }
}
