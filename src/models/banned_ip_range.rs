use chrono::NaiveDateTime;

use crate::schema::banned_ip_ranges;

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(start, end)]
pub struct BannedIpRange {
    pub start: Option<String>,
    pub end: Option<String>,
    pub note: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
