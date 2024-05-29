#![deny(missing_docs)]

use std::num::ParseIntError;

use reqwest::{header::ToStrError, Response};
use thiserror::Error;

/// Errors generated while processing Headers from Requests to the MTG API
#[derive(Clone, Debug, Error)]
pub enum MTGHeaderError {
    /// When an item with a specific name can't be found
    #[error("Header Item Not Found: {n}")]
    ItemMissing {
        /// Name of the item to search for
        n: String,
    },
    #[error("Conversion Error: {e}")]
    /// When any type conversion fails within library
    Conversion {
        /// The wrapped specific conversion error
        e: String,
    },
}

impl From<ToStrError> for MTGHeaderError {
    fn from(value: ToStrError) -> Self {
        MTGHeaderError::Conversion {
            e: value.to_string(),
        }
    }
}

impl From<ParseIntError> for MTGHeaderError {
    fn from(value: ParseIntError) -> Self {
        MTGHeaderError::Conversion {
            e: value.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct MTGHeader {
    link: String,
    page_size: usize,
    count: usize,
    total_count: usize,
    ratelimit_limit: usize,
    ratelimit_remaining: usize,
}

impl MTGHeader {
    pub async fn from_response(res: &Response) -> Result<Self, MTGHeaderError> {
        Ok(MTGHeader {
            link: Self::get_field(res, "Link").await?,
            page_size: Self::get_field(res, "Page-Size").await?.parse()?,
            count: Self::get_field(res, "Count").await?.parse()?,
            total_count: Self::get_field(res, "Total-Count").await?.parse()?,
            ratelimit_limit: Self::get_field(res, "Ratelimit-Limit").await?.parse()?,
            ratelimit_remaining: Self::get_field(res, "Ratelimit-Remaining").await?.parse()?,
        })
    }

    async fn get_field(res: &Response, item: &str) -> Result<String, MTGHeaderError> {
        Ok(res
            .headers()
            .get(item)
            .ok_or(MTGHeaderError::ItemMissing { n: item.to_owned() })?
            .to_str()?
            .to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn construct_header() {
        let res = mtg_api::card_page("0").await.unwrap();
        let header_res = MTGHeader::from_response(&res).await;
        print!("{:?}", header_res);
        assert!(header_res.is_ok());

        let header = header_res.unwrap();
        assert_eq!(header.count, 100);
        assert_eq!(header.page_size, 100);
        assert_eq!(header.total_count, 93643);
        assert_eq!(header.ratelimit_limit, 1000);
        assert!(header.ratelimit_remaining > 0);
    }

    #[test]
    fn conversion_error_parse_int() {
        let err_res = " 12 ".parse::<usize>();
        assert!(err_res.is_err());

        let err = err_res.unwrap_err();
        let header_err: MTGHeaderError = err.clone().into();

        assert_eq!(
            header_err.to_string(),
            format!("Conversion Error: {}", err.to_string())
        );
    }
}
