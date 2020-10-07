//! # Pdf-shape
//!
//! Pdf-shape is a Rust library dedicated to analyse XML files produced by [pdf2xml](https://github.com/kermitt2/pdf2xml)
//!
//! Implemented :
//! - Alignement extraction
//! - Coordinates extraction
//! - Shape extraction
//! - Spacing extraction
//! - Style extraction
//! - Style extraction
//! - Blocks extraction (get all the block elements of a given document)
//! - Texts extraction (get all the text elements of a given document)
//! - Tokens extraction (get all the token elements of a given document)
//!
//! Not yet implemented
//! - Line detection
//! - Column detection
//! - Paragraph detection
//!
//! ## Shape and Spacing
//!
//! The following diagram represents the shape of objects/set of objects and the spacing between objects
//!
//! ![Diagram Shape and Objects](../../../images/shape.svg)
//!
//!
//! ## Line detection
//!
//! A line is a set of objects sharing the same base or a set of objects which are horizontally aligned. Horizontal spacing between objects shouldn't be greater than the horizontal spacing mode of the document.
//!
//! ![Diagram lines detection](../../../images/lines.svg)
//!
//! ## Columns detection
//!
//! ## Paragraph detection
//!
//! A paragraph is a set of lines that are equally spaced vertically. In most cases the paragraph spacing should be greater than the document line spacing. Each paragraph lines have to be vertically aligned.
//!
//! ### Orphans detection
//!
//! ![Diagram orphans detection](../../../images/orphans.svg)
//!

mod raw_document;
mod traits;

pub use raw_document::Document;
pub use traits::*;
