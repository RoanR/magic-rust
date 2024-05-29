//! A library which interacts with [`mtg_api`] deserialising responses into Rust structs

#![deny(missing_docs)]
use reqwest::Response;
use std::fmt;

use colored::Colorize;
use display_cards::{cols, divider, wrap};
use serde::{Deserialize, Serialize};
use thiserror::Error;
mod display_cards;
mod header_cards;

/// Errors generated while making MTG Cards
#[derive(Clone, Debug, Error)]
pub enum MTGCardError {
    #[error("Wrapped API Error: {e}")]
    /// Contains Errors from the [`mtg_api`]
    WrappedAPI {
        /// The Wrapped Error
        e: mtg_api::APIError,
    },
    #[error("Wrapped serde Error: {e}")]
    /// Contains Errors from the deserialisation of Json
    WrappedSerde {
        /// The Wrapped Error
        e: String,
    },
    #[error("No Card Found")]
    /// Error for when no card can be found by given identifier
    NoCardError {},
}

impl From<mtg_api::APIError> for MTGCardError {
    fn from(value: mtg_api::APIError) -> Self {
        MTGCardError::WrappedAPI { e: value }
    }
}

impl From<serde_json::Error> for MTGCardError {
    fn from(value: serde_json::Error) -> Self {
        MTGCardError::WrappedSerde {
            e: value.to_string(),
        }
    }
}

/// An Indiviual Magic The Gathering Card
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Card {
    name: String,
    mana_cost: String,
    #[serde(rename = "type")]
    type_field: String,
    rarity: String,
    set_name: String,
    text: String,
    flavor: String,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let maxl = 50;
        divider(maxl, '*', f)?;

        // Name and Manacost
        cols(&self.name, &self.mana_cost, maxl, f)?;
        divider(maxl, '-', f)?;

        // Types and rarity
        cols(&self.type_field, &self.rarity, maxl, f)?;
        divider(maxl, '-', f)?;

        // Text and Flavour
        wrap(&self.text, maxl, f)?;
        wrap(&self.flavor.italic(), maxl, f)?;
        cols(&"", &self.set_name, maxl, f)?;
        divider(maxl, '*', f)?;
        Ok(())
    }
}

/// Wrapper struct for multiple card responses individual
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct MultiCards {
    /// The cards being wrapped
    pub cards: Vec<Card>,
}

impl MultiCards {
    /// Attempt to convert a [`Response`] into [`MultiCards`]
    pub async fn from_response(res: Response) -> Result<Self, MTGCardError> {
        match mtg_api::check_for_empty(res).await? {
            Some(json) => {
                let res: MultiCards = serde_json::from_str(&json)?;
                Ok(res)
            }
            None => Err(MTGCardError::NoCardError {}),
        }
    }
}

/// Wrapper struct for individual card response
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct IndiCard {
    /// The internal card being wrapped
    pub card: Card,
}

impl IndiCard {
    /// Construct an individual card struct or error if empty
    pub async fn from_response(res: Response) -> Result<Self, MTGCardError> {
        match mtg_api::check_for_empty(res).await? {
            Some(json) => {
                let res: IndiCard = serde_json::from_str(&json)?;
                Ok(res)
            }
            None => Err(MTGCardError::NoCardError {}),
        }
    }
}

/// Takes a card id to find and returns it deserialised into [`IndiCard`]
pub async fn id_find(id: u64) -> Result<IndiCard, MTGCardError> {
    let id_s = id.to_string();
    Ok(IndiCard::from_response(mtg_api::card_id_info(&id_s).await?).await?)
}

/// Takes a card name to find and returns them deserialised into [`MultiCards`]
pub async fn name_find(name: &str) -> Result<MultiCards, MTGCardError> {
    Ok(MultiCards::from_response(mtg_api::card_exact_name_info(name).await?).await?)
}

/// Takes a page number to fetch cards from and returns them deserialised into [`MultiCards`]
pub async fn page_find(number: u64) -> Result<MultiCards, MTGCardError> {
    let index = number.to_string();
    Ok(MultiCards::from_response(mtg_api::card_page(&index).await?).await?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::de::Error;

    #[tokio::test]
    async fn find_card_id() {
        // Get a known card Narset, Enlightened Master
        let a = id_find(386616).await;
        assert!(a.is_ok());

        // Check Individual fields of card struct
        let c = &a.unwrap().card;
        assert_eq!(c.name, "Narset, Enlightened Master");
        assert_eq!(c.mana_cost, "{3}{U}{R}{W}");
        assert_eq!(c.type_field, "Legendary Creature — Human Monk");
        assert_eq!(c.rarity, "Mythic");
        assert_eq!(c.set_name, "Khans of Tarkir");
        assert_eq!(&c.text[..22], "First strike, hexproof");
        assert_eq!(c.flavor, "");

        // Check is Error
        let a = id_find(173132123).await;
        assert!(a.is_err());
    }

    #[tokio::test]
    async fn find_card_name() {
        // Get a known card Narset, Enlightened Master
        let a = name_find("Narset, Enlightened Master").await;
        assert!(a.is_ok());

        // Check it returned the correct card
        let b = id_find(386616).await;
        assert_eq!(a.unwrap().cards[0].name, b.unwrap().card.name);

        // Check it returns an error
        let a = name_find("Narset, Unenlightened Student").await;
        assert!(a.is_err());
    }

    #[test]
    fn display_card() {
        let blank: Card = Card {
            name: "name".to_string(),
            mana_cost: "mana".to_string(),
            type_field: "type".to_string(),
            rarity: "rarity".to_string(),
            set_name: "set".to_string(),
            text: "body".to_string(),
            flavor: "flavour".to_string(),
        };
        let display = "**************************************************\nname                                          mana\n--------------------------------------------------\ntype                                        rarity\n--------------------------------------------------\nbody\nflavour\n                                               set\n**************************************************\n".to_string();
        let blank_display = format!("{}", blank);
        assert_eq!(display, blank_display);
    }

    #[tokio::test]
    async fn find_page() {
        let page_res = page_find(1).await;
        assert!(page_res.is_ok());

        let page = page_res.unwrap();
        assert_eq!(page.cards[0].name, "Ancestor's Chosen");
        assert_eq!(page.cards.len(), 100);

        let page_res = page_find(u64::MAX).await;
        assert!(page_res.is_err());
    }

    #[test]
    fn convert_serde_error() {
        let serde_err = serde_json::Error::custom("Test");
        let mtg_err: MTGCardError = serde_err.into();
        assert_eq!(mtg_err.to_string(), "Wrapped serde Error: Test");
    }
}
