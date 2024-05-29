//! This will allow for accessing data made available by the Magic The Gathering API.
//!
//! See: https://docs.magicthegathering.io/
#![deny(missing_docs)]
use reqwest::{Response, StatusCode};
use thiserror::Error;

/// Base URL of the REST API
const CARDS_URL: &str = "https://api.magicthegathering.io/v1/cards";

/// Errors generated while getting data from MTG api
#[derive(Clone, Debug, Error)]
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
        e: String,
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
        APIError::WrappedReqwest {
            e: value.to_string(),
        }
    }
}

/// Check if there are cards returned in the response
pub async fn check_for_empty(res: Response) -> Result<Option<String>, APIError> {
    let text = res.text().await?;
    if text == "{\"cards\":[]}" {
        Ok(None)
    } else {
        Ok(Some(text))
    }
}

async fn get_request(url: &str) -> Result<Response, APIError> {
    // Perform the GET request
    let response = reqwest::get(url).await?;

    // Check if the request was successful
    match response.status().is_success() {
        true => Ok(response),
        false => Err(APIError::FailedRequest {
            status: response.status(),
        }),
    }
}

/// Find a card by its numerical ID
pub async fn card_id_info(card_id: &str) -> Result<Response, APIError> {
    // Define the URL for the API endpoint
    let url = format!("{}/{}", CARDS_URL, card_id);

    // Perform the GET request
    get_request(&url).await
}

/// Find a card by its exact name
pub async fn card_exact_name_info(card_name: &str) -> Result<Response, APIError> {
    // Define the URL for the API endpoint
    let url = format!("{}?name=\"{}\"", CARDS_URL, card_name);

    // Perform the GET request
    get_request(&url).await
}

/// Get a page of cards
pub async fn card_page(page_number: &str) -> Result<Response, APIError> {
    // Define the URL for the API endpoint
    let url = format!("{}?page={}", CARDS_URL, page_number);

    // Perform the GET request
    get_request(&url).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn fetch_id_result() {
        let pass = card_id_info("386616");
        let fail = card_id_info("as32as");
        assert!(pass.await.is_ok());
        assert!(fail.await.is_err());
    }

    #[tokio::test]
    async fn fetch_name_result() {
        let exact_pass = card_exact_name_info("Narset, Enlightened Master").await;
        let exact_fail = card_exact_name_info("Narset, Unelightned Student").await;
        // Check internal pass
        assert!(exact_pass.is_ok());
        let exact_pass_res = exact_pass.unwrap();
        assert!(check_for_empty(exact_pass_res).await.unwrap().is_some());
        // Check internal err
        assert!(exact_fail.is_ok());
        let exact_fail_res = exact_fail.unwrap();
        assert!(check_for_empty(exact_fail_res).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn fetch_page_header() {
        let page_pass = card_page("1").await;
        assert!(page_pass.is_ok());
        let response = page_pass.unwrap();

        // Check Header
        let headers = response.headers();
        assert_eq!(48, headers.capacity());
        // Check total-count
        assert!(headers.get("total-count").is_some());
        let total_count = headers.get("total-count").unwrap().to_str().unwrap();
        assert_eq!("93643", total_count);
    }
}
