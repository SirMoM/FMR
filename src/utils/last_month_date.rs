use chrono::{DateTime, Datelike, Months, Timelike, Utc};

pub fn last_month_date() -> Option<DateTime<Utc>> {
    let res: DateTime<Utc> = Utc::now().checked_sub_months(Months::new(0))?;
    res.with_day(1)?
        .with_hour(0)?
        .with_minute(0)?
        .with_second(0)?;
    return Some(res);
}
