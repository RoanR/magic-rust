//! A library which interacts with [`mtg_api`] deserialising responses into Rust structs

#![deny(missing_docs)]
use std::fmt;

use colored::Colorize;
use serde::{Deserialize, Serialize};
use thiserror::Error;

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

impl Card {
    fn display_divider(max: usize, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for _ in 0..max {
            write!(f, "-")?;
        }
        Ok(())
    }
    fn display_cols(
        left: &str,
        right: &str,
        max: usize,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let mut pad = " ".to_string();
        while pad.len() + left.len() + right.len() < max {
            pad += " ";
        }
        write!(f, "\n{}{}{}\n", left, pad, right)?;
        Ok(())
    }
    fn display_wrap(body: &str, max: usize, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut count = 0;
        for ch in body.chars() {
            if ch == '\n' {
                count = 0;
                write!(f, "{}", ch)?;
            } else if count % max == 0 {
                write!(f, "\n{}", ch)?;
                count += 1;
            } else {
                write!(f, "{}", ch)?;
                count += 1;
            }
        }
        write!(f, "\n")?;
        Ok(())
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let maxl = 50;
        Card::display_divider(maxl, f)?;

        // Name and Manacost
        Card::display_cols(&self.name, &self.mana_cost, maxl, f)?;
        Card::display_divider(maxl, f)?;

        // Types and rarity
        Card::display_cols(&self.type_field, &self.rarity, maxl, f)?;
        Card::display_divider(maxl, f)?;

        // Text and Flavour
        Card::display_wrap(&self.text, maxl, f)?;
        Card::display_wrap(&self.flavor.italic(), maxl, f)?;
        Card::display_cols(&"", &self.set_name, maxl, f)?;
        Card::display_divider(maxl, f)?;
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
