use std::borrow::Cow;

use chrono::NaiveDate;

pub fn date_or_today<'a>(date: Option<NaiveDate>) -> Cow<'a, str> {
    date.map_or(Cow::Borrowed("today"), |d| {
        Cow::Owned(d.format("%Y-%m-%d").to_string())
    })
}
