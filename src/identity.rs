use chrono::{DateTime, Utc, FixedOffset};

#[derive(Debug)]
pub struct Identity {
    name: Vec<u8>,
    email: Vec<u8>,
    at: DateTime<Utc>,
    offset: FixedOffset
}