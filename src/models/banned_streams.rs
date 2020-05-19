use chrono::NaiveDateTime;

use crate::schema::banned_streams;

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(channel, service)]
pub struct BannedStream {
    pub channel: String,
    pub service: String,
    pub reason: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
