use serde::Serialize;
use time::{OffsetDateTime, PrimitiveDateTime};

#[derive(Serialize)]

pub struct DateTime {}

impl DateTime {
    pub fn now() -> PrimitiveDateTime {
        PrimitiveDateTime::new(
            OffsetDateTime::now_utc().date(),
            OffsetDateTime::now_utc().time(),
        )
    }
}
