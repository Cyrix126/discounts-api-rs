pub mod error;
use chrono::{DateTime, Utc};
use derive_more::derive::Deref;
use discounts_common::Discount;
use error::DiscountClientError;
use reqwest::{Client as ReqClient, ClientBuilder, Url};
#[derive(Deref, Clone)]
pub struct Client {
    #[deref]
    client: ReqClient,
    default_url: Url,
}

impl Client {
    pub fn new(uri: Url) -> Self {
        Client {
            client: ClientBuilder::new().build().unwrap(),
            default_url: uri,
        }
    }
    pub async fn create_discount(
        &self,
        code: String,
        percentage: u8,
        date_begin: Option<DateTime<Utc>>,
        date_end: Option<DateTime<Utc>>,
    ) -> Result<u32, DiscountClientError> {
        if percentage > 100 {
            return Err(DiscountClientError::Percentage);
        }
        let discount = Discount {
            id: 0,
            code,
            percentage: percentage as i16,
            date_begin,
            date_end,
        };
        let url = format!("{}/discounts", self.default_url);
        Ok(self
            .post(url)
            .json(&discount)
            .send()
            .await?
            .text()
            .await?
            .parse::<u32>()?)
    }
    pub async fn update_discount(&self, discount: Discount) -> Result<(), DiscountClientError> {
        if discount.percentage > 100 {
            return Err(DiscountClientError::Percentage);
        }
        let url = format!("{}/discounts/{}", self.default_url, discount.id);
        self.put(url).json(&discount).send().await?;
        Ok(())
    }
    pub async fn read_discount(&self, id: u32) -> Result<Discount, DiscountClientError> {
        let url = format!("{}/discounts/{}", self.default_url, id);
        Ok(self.get(url).send().await?.json::<Discount>().await?)
    }
    pub async fn delete_discount(&self, id: u32) -> Result<(), DiscountClientError> {
        let url = format!("{}/discounts/{}", self.default_url, id);
        self.delete(url).send().await?;
        Ok(())
    }
    pub async fn check_code_validity(&self, code: &str) -> Result<(), DiscountClientError> {
        let url = format!("{}/check_validity/{}", self.default_url, code);
        self.get(url).send().await?;
        Ok(())
    }
}
