use chrono::NaiveDate;
use serde::Deserialize;

use crate::{models::body::weight::WeightLog, Client, Result};

pub struct BodyHandler<'client> {
    client: &'client Client,
}

impl<'client> BodyHandler<'client> {
    pub fn new(client: &'client Client) -> Self {
        Self { client }
    }

    pub async fn get_weight_log(
        &self,
        date: &NaiveDate,
        user_id: Option<&str>,
    ) -> Result<Vec<WeightLog>> {
        let user_id = user_id.unwrap_or("-");
        let date = date.format("%Y-%m-%d");

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
}
