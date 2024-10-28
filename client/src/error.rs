use std::num::ParseIntError;

use thiserror::Error;
#[derive(Error, Debug)]
pub enum DiscountClientError {
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error("Percentage value not valid")]
    Percentage,
    #[error("response is not expected")]
    Parse(#[from] ParseIntError),
    #[error("Code not valid")]
    CodeInvalid,
}
