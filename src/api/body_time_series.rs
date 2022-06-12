use std::borrow::Cow;

use chrono::NaiveDate;
use serde::Deserialize;

use crate::{models::body::weight::WeightLog, Client, Result};

pub struct BodyTimeSeriesHandler<'client> {
    client: &'client Client,
}

impl<'client> BodyTimeSeriesHandler<'client> {
    pub fn new(client: &'client Client) -> Self {
        Self { client }
    }

    pub async fn get_weight_time_series_by_date_range(
        &self,
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>,
        user_id: Option<&str>,
    ) -> Result<Vec<WeightLog>> {
        let user_id = user_id.unwrap_or("-");
        let start_date = formatted_or_today(start_date);
        let end_date = formatted_or_today(end_date);

        #[derive(Deserialize)]
        struct Response {
            pub weight: Vec<WeightLog>,
        }
        let response: Response = self
            .client
            .get(
                &format!("/1/user/{user_id}/body/log/weight/date/{start_date}/{end_date}.json"),
                None::<&()>,
            )
            .await?;
        Ok(response.weight)
    }
}

fn formatted_or_today<'a>(date: Option<NaiveDate>) -> Cow<'a, str> {
    date.map_or(Cow::Borrowed("today"), |d| {
        Cow::Owned(d.format("%Y-%m-%d").to_string())
    })
}
