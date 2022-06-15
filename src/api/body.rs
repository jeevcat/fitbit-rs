use chrono::{NaiveDate, Utc};
use serde::Deserialize;

use crate::{
    models::body::{fat::FatLog, weight::WeightLog},
    util::date_or_today,
    Client, Result,
};

pub struct BodyHandler<'client> {
    client: &'client Client,
}

impl<'client> BodyHandler<'client> {
    pub fn new(client: &'client Client) -> Self {
        Self { client }
    }

    pub async fn get_weight_log(
        &self,
        date: Option<NaiveDate>,
        user_id: Option<&str>,
    ) -> Result<Vec<WeightLog>> {
        let user_id = user_id.unwrap_or("-");
        let date = date_or_today(date);

        #[derive(Deserialize)]
        struct Response {
            pub weight: Vec<WeightLog>,
        }
        let response: Response = self
            .client
            .get(
                &format!("/1/user/{user_id}/body/log/weight/date/{date}.json"),
                None::<&()>,
            )
            .await?;
        Ok(response.weight)
    }

    pub async fn get_body_fat_log(
        &self,
        date: Option<NaiveDate>,
        user_id: Option<&str>,
    ) -> Result<Vec<FatLog>> {
        let user_id = user_id.unwrap_or("-");
        let date = date
            .unwrap_or_else(|| Utc::now().naive_utc().date())
            .format("%Y-%m-%d")
            .to_string();

        #[derive(Deserialize)]
        struct Response {
            pub fat: Vec<FatLog>,
        }
        let response: Response = self
            .client
            .get(
                &format!("/1/user/{user_id}/body/log/fat/date/{date}.json"),
                None::<&()>,
            )
            .await?;
        Ok(response.fat)
    }
}
