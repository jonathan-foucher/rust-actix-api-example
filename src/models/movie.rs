use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Movie {
    pub id: u32,
    pub title: String,
    pub release_date: NaiveDate
}
