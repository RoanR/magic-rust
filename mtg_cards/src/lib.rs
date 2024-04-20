//! A library which interacts with [`mtg_api`] deserialising responses into Rust structs

#![deny(missing_docs)]
use std::fmt;

use colored::Colorize;
use display_cards::{cols, divider, wrap};
use serde::{Deserialize, Serialize};
use thiserror::Error;

mod display_cards;
/// Errors generated while making MTG Cards
#[derive(Debug, Error)]
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
        e: serde_json::Error,
    },
}

impl From<mtg_api::APIError> for MTGCardError {
    fn from(value: mtg_api::APIError) -> Self {
        MTGCardError::WrappedAPI { e: value }
    }
}

impl From<serde_json::Error> for MTGCardError {
    fn from(value: serde_json::Error) -> Self {
        MTGCardError::WrappedSerde { e: value }
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

impl Card {}

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

/// Wrapper struct for for individual
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Cards {
    /// The internal card being wrapped
    pub card: Card,
}

/// Takes a card id to find and returns it deserialised into [`Cards`]
pub async fn id_find(id: u64) -> Result<Cards, MTGCardError> {
    let id_s = id.to_string();
    let json = mtg_api::card_id_info(&id_s).await?;
    let res: Cards = serde_json::from_str(&json)?;
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn find_card() {
        // Get a known card Narset, Enlightened Master
        let a = id_find(386616).await;
        assert!(a.is_ok());

        // Check Individual fields of card struct
        let c = a.unwrap().card;
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
}
