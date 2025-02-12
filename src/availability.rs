use crate::DATE_FORMAT;
use chrono::{Datelike, NaiveDate, NaiveDateTime, Utc};
use std::collections::HashSet;

pub struct Availability {
    dates: HashSet<NaiveDateTime>,
}

impl Availability {
    pub fn new() -> Self {
        Availability {
            dates: HashSet::new(),
        }
    }

    pub fn insert(&mut self, v: NaiveDateTime) -> bool {
        self.dates.insert(v)
    }

    pub fn remove(&mut self, k: &NaiveDateTime) -> bool {
        self.dates.remove(k)
    }

    pub fn get_all_availability_datetime_string(&mut self) -> Vec<String> {
        let mut datetimes = vec![];
        let today =
            NaiveDate::from_ymd_opt(Utc::now().year(), Utc::now().month(), Utc::now().day())
                .unwrap();

        // 昨日以前のものは削除する。
        for &datetime in &self.dates {
            if datetime.date() >= today {
                datetimes.push(datetime);
            }
        }

        self.dates = datetimes.clone().into_iter().collect();

        datetimes.sort();
        datetimes
            .into_iter()
            .map(|datetime| datetime.format(DATE_FORMAT).to_string())
            .collect()
    }
}
