//! This will allow for accessing data made available by the Magic The Gathering API.
//!
//! See: https://docs.magicthegathering.io/
#![deny(missing_docs)]
use reqwest::StatusCode;
use thiserror::Error;

/// Errors generated while getting data from MTG api
#[derive(Debug, Error)]
pub enum APIError {
    /// When Get request fails
    #[error("Get Request failed with status code: {status}")]
    FailedRequest {
        /// The status code returned by the request
        status: StatusCode,
    },
    #[error("Wrapped Reqwest Error: {e}")]
    /// Contain other misc errors from [`reqwest`] crate
    WrappedReqwest {
        /// The Wrapped Error
        e: reqwest::Error,
    },
}

impl From<reqwest::Error> for APIError {
    fn from(value: reqwest::Error) -> Self {
        APIError::WrappedReqwest { e: value }
    }
}

#[allow(dead_code)]
async fn fetch_card_info(card_id: &str) -> Result<String, APIError> {
    // Define the URL for the API endpoint
    let url = format!("https://api.magicthegathering.io/v1/cards/{}", card_id);

    // Perform the GET request
    let response = reqwest::get(&url).await?;

    // Check if the request was successful
    match response.status().is_success() {
        true => Ok(response.text().await?),
        false => Err(APIError::FailedRequest {
            status: response.status(),
        }),
    }
}

// fn main() -> Result<()> {
//     // Call the async function to fetch and print card info with a specific card ID
//     let a = task::block_on(fetch_card_info("386616"))?;
//     println!("{}", a);
//     Ok(())
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn fetch_card_result() {
        assert!(fetch_card_info("386616").await.is_ok());
        assert!(fetch_card_info("as32as").await.is_err());
    }
}
