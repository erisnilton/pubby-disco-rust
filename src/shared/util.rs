use chrono::Timelike;

pub fn trim_datetime(date: chrono::NaiveDateTime) -> chrono::NaiveDateTime {
  date.with_nanosecond(0).unwrap()
}

pub fn naive_now() -> chrono::NaiveDateTime {
  trim_datetime(chrono::Utc::now().naive_utc())
}
