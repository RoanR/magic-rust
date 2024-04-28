//! This will allow for accessing data made available by the Magic The Gathering API.
//!
//! See: https://docs.magicthegathering.io/
#![deny(missing_docs)]
use reqwest::StatusCode;
use thiserror::Error;

/// Base URL of the REST API
const CARDS_URL: &str = "https://api.magicthegathering.io/v1/cards";

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
    #[error("No Cards exist with name: {name}")]
    /// When partial search returns no cards
    NoSuchCardName {
        /// The name searched for
        name: String,
    },
}

impl From<reqwest::Error> for APIError {
    fn from(value: reqwest::Error) -> Self {
        APIError::WrappedReqwest { e: value }
    }
}

/// Find a card by its numerical ID
pub async fn card_id_info(card_id: &str) -> Result<String, APIError> {
    // Define the URL for the API endpoint
    let url = format!("{}/{}", CARDS_URL, card_id);

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

/// Find a card by its exact name
#[allow(dead_code)]
pub async fn card_exact_name_info(card_name: &str) -> Result<String, APIError> {
    let url = format!("{}?name=\"{}\"", CARDS_URL, card_name);
    println!("{url}");
    let response = reqwest::get(&url).await?;

    // Check the request was successful
    if response.status().is_success() {
        let text = response.text().await?;

        // Check the cards returned are non-empty
        if text == "{\"cards\":[]}" {
            return Err(APIError::NoSuchCardName {
                name: card_name.to_string(),
            });
        } else {
            return Ok(text);
        }
    } else {
        return Err(APIError::FailedRequest {
            status: response.status(),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn fetch_card_result() {
        let pass = card_id_info("386616");
        let fail = card_id_info("as32as");
        assert!(pass.await.is_ok());
        assert!(fail.await.is_err());
    }

    #[tokio::test]
    async fn fetch_name_result() {
        let exact_pass = card_exact_name_info("Narset, Enlightened Master").await;

        //let fail = card_name_info("FAKE CARD NAME").await;
        assert!(exact_pass.is_ok());
        //assert!(part_pass.is_ok());
        println!("{}", exact_pass.unwrap());
    }
}
