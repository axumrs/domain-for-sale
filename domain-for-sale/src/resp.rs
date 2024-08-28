use serde::Serialize;

use crate::Error;

#[derive(Serialize)]
pub struct Resp<T> {
    pub code: i32,
    pub msg: String,
    pub data: T,
}

impl<T: Serialize> Resp<T> {
    pub fn ok(data: T) -> Self {
        Self {
            code: 0,
            msg: "OK".to_string(),
            data,
        }
    }
}

impl Resp<()> {
    pub fn ok_empty() -> Self {
        Self::ok(())
    }

    pub fn err(e: Error) -> Self {
        Self {
            code: -1,
            msg: e.msg(),
            data: (),
        }
    }
}
