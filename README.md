<div align="center">

# PDF-shape

[![Project Status: Concept – Minimal or no implementation has been done yet, or the repository is only intended to be a limited example, demo, or proof-of-concept.](https://www.repostatus.org/badges/latest/concept.svg)](https://www.repostatus.org/#concept)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![contributions welcome](https://img.shields.io/badge/contributions-welcome-brightgreen.svg?style=flat)]()

</div>

PDF-shape is a Rust library dedicated to analyse XML files produced by pdf2xml

## Features

Implemented :

- Alignement extraction
- Coordinates extraction
- Shape extraction
- Spacing extraction
- Style extraction
- Style extraction
- Blocks extraction (get all the block elements of a given document)
- Texts extraction (get all the text elements of a given document)
- Tokens extraction (get all the token elements of a given document)

Not implemented yet:

- Line detection
- Column detection
- Paragraph detection
- Blocks detection

## Examples

You can run the example with :

```
cargo run --example=main
```

## Documentation

You can build the documentation with :

```
cargo doc --open --lib --no-deps
```

## Concepts

### Shape (width/heigt) and spacing (vertical/horizontal)

The following diagram represents the shape of objects/set of objects and the spacing between them

![Diagram Shape and Objects](./images/shape.svg)

### Line detection

A line is a set of objects sharing the same base or a set of objects which are horizontally aligned. Horizontal spacing between objects shouldn't be greater than the horizontal spacing mode of the document.

![Diagram lines detection](./images/lines.svg)

### Columns detection

### Paragraph detection

A paragraph is a set of lines that are equally spaced vertically. In most cases the paragraph spacing should be greater than the document line spacing. Each paragraph lines have to be vertically aligned.

#### Orphans detection

![Diagram orphans detection](./images/orphans.svg)
