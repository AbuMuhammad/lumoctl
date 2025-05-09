use chrono::{NaiveDate, NaiveTime};

#[derive(Queryable)]
pub struct JadwalDepok {
    pub tanggal: NaiveDate,
    pub fajr: Option<NaiveTime>,
    pub syuruq: Option<NaiveTime>,
    pub zhuhr: Option<NaiveTime>,
    pub ashar: Option<NaiveTime>,
    pub maghrib: Option<NaiveTime>,
    pub isya: Option<NaiveTime>,
}
