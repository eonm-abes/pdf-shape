//! This module is used to deserialize XML files produced by [pdf2xml](https://github.com/kermitt2/pdf2xml).
//!
//! You should use the `-blocks` arg of pdf2xml to produce files that can be deserialize with rythes appropriate XML files.

use crate::traits::{Alignement, Coordinates, Shape, Style};

use serde::Deserialize;
use std::fmt;

/// A struct representing a set of tokens
#[derive(Debug, Clone, Deserialize)]
#[serde(transparent)]
pub struct DeserizalizationTokens(pub Vec<Token>);

impl fmt::Display for DeserizalizationTokens {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|e| { e.value.clone().unwrap_or("".to_string()) })
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl IntoIterator for DeserizalizationTokens {
    type Item = Token;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl DeserizalizationTokens {
    pub fn new(input: Vec<Token>) -> DeserizalizationTokens {
        DeserizalizationTokens(input)
    }
}

/// A struct representing the XML document created by xml2pdf
#[derive(Debug, Deserialize, Clone)]
pub struct Document {
    #[serde(alias = "BLOCK")]
    blocks: Vec<Block>,
}

impl Document {
    /// Returns all the blocks elements of a document
    pub fn get_blocks(&self) -> Vec<Block> {
        self.blocks.clone()
    }

    /// Returns all the blocks elements of a document in a borrowed manner
    pub fn get_blocks_borrowed(&self) -> Vec<&Block> {
        let mut borowed_block = vec![];

        for block in &self.blocks {
            borowed_block.push(block)
        }

        borowed_block
    }

    /// Returns all the text elements of a document
    pub fn get_texts(&self) -> Vec<Text> {
        self.get_blocks()
            .iter()
            .flat_map(|block| block.texts.clone())
            .collect::<Vec<Text>>()
    }

    /// Returns all the text elements of a document in a borrowed manner
    pub fn get_texts_borrowed(&self) -> Vec<&Text> {
        self.get_blocks_borrowed()
            .iter()
            .flat_map(|block| &block.texts)
            .collect::<Vec<&Text>>()
    }

    /// Returns all the tokens elements of a document
    pub fn get_tokens(&self) -> DeserizalizationTokens {
        DeserizalizationTokens(
            self.get_texts()
                .iter()
                .flat_map(|text| text.tokens.0.clone())
                .collect::<Vec<Token>>(),
        )
    }

    pub fn get_fsm_tokens(&self) -> Tokens {
        Tokens {
            tokens: self
                .get_texts_borrowed()
                .iter()
                .flat_map(|text| &text.tokens.0)
                .collect::<Vec<&Token>>(),
        }
    }
}

/// A struct representing a block. Block holds text elements
#[derive(Debug, Deserialize, Clone)]
pub struct Block {
    #[serde(alias = "TEXT")]
    texts: Vec<Text>,
    id: String,
    x: f32,
    y: f32,
    height: Option<f32>,
    width: Option<f32>,
}

/// A struct representing a text element of a Document
///
/// A text element that holds tokens
#[derive(Debug, Deserialize, Clone)]
pub struct Text {
    x: f32,
    y: f32,
    id: String,
    width: f32,
    height: f32,
    #[serde(alias = "TOKEN")]
    tokens: DeserizalizationTokens,
}

/// A struct representing a token element of a Document
///
/// A token represents a piece of text
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Token {
    sid: Option<String>,
    id: String,
    font_name: Option<String>,
    bold: bool,
    italic: bool,
    font_color: String,
    font_size: f32,
    rotation: f32,
    angle: f32,
    pub x: f32,
    pub y: f32,
    pub base: f32,
    pub width: f32,
    height: f32,
    #[serde(rename = "$value")]
    pub value: Option<String>,
}

impl Coordinates for Token {
    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }

    fn base(&self) -> f32 {
        self.base
    }
}

impl Shape for Token {
    fn width(&self) -> f32 {
        self.width
    }

    fn height(&self) -> f32 {
        self.height
    }

    fn rotation(&self) -> Option<f32> {
        Some(self.rotation)
    }

    fn angle(&self) -> Option<f32> {
        Some(self.angle)
    }
}

impl Style for Token {
    fn font_size(&self) -> Option<f32> {
        Some(self.font_size)
    }

    fn avg_font_size(&self) -> Option<f32> {
        Some(self.font_size)
    }
    fn bold(&self) -> Option<bool> {
        Some(self.bold)
    }

    fn italic(&self) -> Option<bool> {
        Some(self.italic)
    }
}

impl Alignement for Token {}

#[derive(Debug, Clone)]
pub struct Tokens<'a> {
    pub tokens: Vec<&'a Token>,
}

impl<'a> Tokens<'a> {
    fn first_token(&self) -> Option<&Token> {
        self.tokens.first().map(|t| *t)
    }

    fn last_token(&self) -> Option<&Token> {
        self.tokens.last().map(|t| *t)
    }

    fn widest_token(&self) -> Option<&Token> {
        let mut tokens = self.tokens.clone();

        tokens.sort_by(|a, b| a.width.partial_cmp(&b.width).unwrap());
        tokens.last().map(|t| *t)
    }
}

impl<'a> IntoIterator for Tokens<'a> {
    type Item = &'a Token;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.tokens.into_iter()
    }
}

impl<'a> Coordinates for Tokens<'a> {
    fn x(&self) -> f32 {
        if let Some(first_token) = self.first_token() {
            first_token.x
        } else {
            0.0
        }
    }

    fn y(&self) -> f32 {
        if let Some(first_token) = self.first_token() {
            first_token.y
        } else {
            0.0
        }
    }

    /// Takes the lowest base among a collection of token
    fn base(&self) -> f32 {
        use std::cmp::Ordering::Equal;

        let mut bases = self
            .tokens
            .iter()
            .map(|token| token.base())
            .collect::<Vec<f32>>();

        bases.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));

        if let Some(base) = bases.first() {
            *base
        } else {
            0.0
        }
    }
}
