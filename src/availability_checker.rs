use crate::{
    availability::Availability, browser_handler::BrowserHandler, notifier::DiscordNotifier,
    AVAILABLE_STATUS, CLASS_PERIOD, DATE_FORMAT, ONE_WEEK, RESERVED_STATUS,
};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use std::error::Error;

pub struct AvailabilityChecker {
    browser: BrowserHandler,
    availability: Availability,
    notifier: DiscordNotifier,
}

impl AvailabilityChecker {
    pub fn new(browser: BrowserHandler, notifier: DiscordNotifier) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            browser,
            availability: Availability::new(),
            notifier,
        })
    }

    pub async fn check_all_availability(&mut self) -> Result<(), Box<dyn Error>> {
        while self.browser.navigate_next_day().is_ok() {
            self.check_availability().await?
        }

        while self.browser.navigate_prev_day().is_ok() {
            self.check_availability().await?
        }

        Ok(())
    }

    async fn check_availability(&mut self) -> Result<(), Box<dyn Error>> {
        let start_date = self.browser.get_start_date()?;
        let table_vec = self.browser.get_table_vec()?;

        let available_datetime = self.get_availability(start_date, &table_vec);
        let new_available_datetime = self.get_new_available_datetime_string(available_datetime);

        let unavailable_datetime = self.get_unavailable_datetime(start_date, &table_vec);
        let unavailable_datetime_reserved_by_others =
            self.get_unavailable_datetime_string_reserved_by_others(unavailable_datetime);

        if new_available_datetime.is_empty() && unavailable_datetime_reserved_by_others.is_empty() {
            return Ok(());
        }

        let all_available_datetime = self.availability.get_all_availability_datetime_string();

        let mut message = "## 予約可能日時\n".to_string();

        if !new_available_datetime.is_empty() {
            message.push_str(&format!("new: {}\n", new_available_datetime.join(", ")));
        }

        message.push_str(&format!("{}", all_available_datetime.join(", ")));

        self.notifier.send(message).await?;

        Ok(())
    }

    fn get_availability(
        &self,
        start_date: NaiveDate,
        table_vec: &Vec<Vec<String>>,
    ) -> Vec<NaiveDateTime> {
        let mut available_datetime = vec![];

        for i in 0..ONE_WEEK {
            let date = start_date + chrono::Duration::days(i as i64);

            for j in 0..CLASS_PERIOD {
                let datetime =
                    NaiveDateTime::new(date, NaiveTime::from_hms_opt(9 + j as u32, 0, 0).unwrap());

                let Some(table_vec_i) = table_vec.get(i) else {
                    continue;
                };
                let Some(table_vec_ij) = table_vec_i.get(j) else {
                    continue;
                };

                if table_vec_ij == AVAILABLE_STATUS {
                    available_datetime.push(datetime);
                }
            }
        }

        available_datetime
    }

    fn get_new_available_datetime_string(
        &mut self,
        available_datetime: Vec<NaiveDateTime>,
    ) -> Vec<String> {
        let mut new_available_datetime = vec![];

        for datetime in available_datetime {
            if self.availability.insert(datetime) {
                new_available_datetime.push(datetime.format(DATE_FORMAT).to_string());
            }
        }

        new_available_datetime
    }

    fn get_unavailable_datetime(
        &self,
        start_date: NaiveDate,
        table_vec: &Vec<Vec<String>>,
    ) -> Vec<NaiveDateTime> {
        let mut datetime_reserved_by_others = vec![];

        for i in 0..ONE_WEEK {
            let date = start_date + chrono::Duration::days(i as i64);

            for j in 0..CLASS_PERIOD {
                let datetime =
                    NaiveDateTime::new(date, NaiveTime::from_hms_opt(9 + j as u32, 0, 0).unwrap());

                let Some(table_vec_i) = table_vec.get(i) else {
                    continue;
                };
                let Some(table_vec_ij) = table_vec_i.get(j) else {
                    continue;
                };

                if table_vec_ij == RESERVED_STATUS {
                    datetime_reserved_by_others.push(datetime);
                }
            }
        }

        datetime_reserved_by_others
    }

    fn get_unavailable_datetime_string_reserved_by_others(
        &mut self,
        unavailable_datetime: Vec<NaiveDateTime>,
    ) -> Vec<String> {
        let mut unavailable_datetime_reserved_by_others = vec![];

        for datetime in unavailable_datetime {
            if self.availability.remove(&datetime) {
                unavailable_datetime_reserved_by_others
                    .push(datetime.format(DATE_FORMAT).to_string());
            }
        }

        unavailable_datetime_reserved_by_others
    }
}
