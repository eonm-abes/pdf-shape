//! This module contains all the Traits that can be used to work with objects or set of objects
//!
//! # Shape and Spacing
//!
//! The following diagram represents the shape of objects/set of objects and the spacing between objects
//!
//! ![Diagram Shape and Objects](../../../../images/shape.svg)
//!
//!
//! ## Line detection
//!
//! A line is a set of objects sharing the same base or a set of objects which are horizontally aligned. Horizontal spacing between objects shouldn't be greater than the horizontal spacing mode of the document.
//!
//! ![Diagram lines detection](../../../../images/lines.svg)
//!
//! ## Paragraph detection
//!
//! A paragraph is a set of lines that are equally spaced vertically. In most cases the paragraph spacing should be greater than the document line spacing. Each paragraph lines have to be horizontally aligned.
//!
//! ## Columns detection
//!
//! ## Orphans detection
//!
//! ![Diagram orphans detection](../../../../images/orphans.svg)

/// Get the absolute coordinates of an object or a set of objects
pub trait Coordinates {
    /// This method returns the y position of an object or a set of objects
    fn y(&self) -> f32;
    /// This method returns the x position of an object or a set of objects
    fn x(&self) -> f32;
    /// This method returns the base position of an object or a set of objects
    fn base(&self) -> f32;
}

/// Get the shape of an object or a set of objects
pub trait Shape {
    /// This method returns the width of an object or a set of objects
    fn width(&self) -> f32;
    /// This method returns the height of an object or a set of objects
    fn height(&self) -> f32;
    /// This method returns the rotation of an object. None is always returned for a set of objects
    fn rotation(&self) -> Option<f32>;
    /// This method returns the angle of an object. None is always returned for a set of objects
    fn angle(&self) -> Option<f32>;
}

/// Get the style of an object
pub trait Style {
    /// This method returns the font size of an object or a set of objects
    ///
    /// ⚠️ This method returns `None` if the font size of an object is unknown or if all the objects of a set doesn't have the same font size
    fn font_size(&self) -> Option<f32> {
        None
    }

    /// This method returns the average font size of an object or a set of objects
    fn avg_font_size(&self) -> Option<f32> {
        None
    }

    /// This methods returns the font color of an object.
    ///
    /// ⚠️ This methods returns `None` if the color of an object is unknown or if all objects in a set doesn't have the same color
    fn font_color(&self) -> Option<String> {
        None
    }

    /// This method checks if a object or a set of objects are bold or not
    ///
    /// ⚠️ This method returns `None` if the font weight of an object is unknown or if all the objects of a set doesn't have the same font weight
    fn bold(&self) -> Option<bool> {
        None
    }

    /// This method checks if a object or a set of objects are italic or not
    fn italic(&self) -> Option<bool> {
        None
    }
}

impl<'a, OBJECTSET, OBJECT> Style for OBJECTSET
where
    OBJECTSET: IntoIterator<Item = &'a OBJECT> + Clone,
    OBJECT: 'a + Style,
{
    fn font_size(&self) -> Option<f32> {
        let objects = self.clone().into_iter();

        let mut last_font_size: Option<f32> = None;

        for object in objects {
            match last_font_size {
                Some(_) => {
                    if object.font_size() != last_font_size {
                        return None;
                    }
                }
                None => last_font_size = object.font_size(),
            }
        }

        last_font_size
    }

    fn avg_font_size(&self) -> Option<f32> {
        let objects = self.clone().into_iter();

        let mut o = objects.map(|object| object.font_size());

        if o.all(|object| object.is_some()) {
            Some(stats::mean(o.flat_map(|object| object)) as f32)
        } else {
            None
        }
    }

    fn bold(&self) -> Option<bool> {
        let objects = self.clone().into_iter();

        let mut last_bold: Option<bool> = None;

        for object in objects {
            match last_bold {
                Some(_) => {
                    if object.bold() != last_bold {
                        return None;
                    }
                }
                None => last_bold = object.bold(),
            }
        }

        last_bold
    }

    fn italic(&self) -> Option<bool> {
        let objects = self.clone().into_iter();

        let mut last_italic: Option<bool> = None;

        for object in objects {
            match last_italic {
                Some(_) => {
                    if object.italic() != last_italic {
                        return None;
                    }
                }
                None => last_italic = object.italic(),
            }
        }

        last_italic
    }
}

impl<'a, OBJECTSET, OBJECT> Shape for OBJECTSET
where
    OBJECTSET: IntoIterator<Item = &'a OBJECT> + Clone,
    OBJECT: 'a + Coordinates + Shape,
{
    fn width(&self) -> f32 {
        // Takes the token with the lowest x = lower bound
        // Takes the token for which the sum of x.position + self.width is higher = upper bound
        use std::cmp::Ordering::Equal;
        let tokens = self.clone().into_iter();

        let mut widths = tokens
            .map(|token| (token.x(), token.width()))
            .collect::<Vec<(f32, f32)>>();

        widths.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Equal));

        let lower_bound = match widths.first() {
            Some(token) => token.0,
            None => 0.0,
        };

        let mut widths = widths
            .iter()
            .map(|(x_position, width)| x_position + width)
            .collect::<Vec<f32>>();

        widths.sort_by(|a, b| a.partial_cmp(&b).unwrap_or(Equal));

        let upper_bound = match widths.last() {
            Some(width) => *width,
            None => 0.0,
        };

        upper_bound - lower_bound
    }

    fn height(&self) -> f32 {
        use std::cmp::Ordering::Equal;
        let tokens = self.clone().into_iter();

        let mut heights = tokens
            .map(|token| (token.y(), token.height()))
            .collect::<Vec<(f32, f32)>>();

        heights.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Equal));

        let upper_bound = match heights.last() {
            Some(token) => token.0,
            None => 0.0,
        };

        let mut heights = heights
            .iter()
            .map(|(y_position, height)| y_position - height)
            .collect::<Vec<f32>>();

        heights.sort_by(|a, b| a.partial_cmp(&b).unwrap_or(Equal));

        let lower_bound = match heights.first() {
            Some(height) => *height,
            None => 0.0,
        };

        upper_bound - lower_bound
    }

    fn rotation(&self) -> Option<f32> {
        None
    }

    fn angle(&self) -> Option<f32> {
        None
    }
}

#[derive(PartialEq)]
/// Represents all possible alignements between a set of objects
pub enum ObjectAlignement {
    Alinged, // Aligned in x y
    /// Objects are aligned horizontaly
    /// ```
    ///  +--------+  +---------+
    ///  |........|  |.........|
    /// →+--------+ →+---------+
    /// ```
    HorizontalAligned, // Aligned in y
    /// Objects are aligned horizontaly and are verticaly centered with each others
    /// ```
    ///              +-------------+
    ///  +--------+  |             |
    /// →|    +   | →|      +      |
    ///  +--------+  |             |
    ///              +-------------+
    /// ```
    HorizontalCenterAligned,
    /// Objects are verticaly aligned by their left side
    /// ```
    ///  ↓
    ///  +---------+
    ///  |         |
    ///  +---------+
    ///  ↓
    ///  +------+
    ///  |      |
    ///  +------+
    /// ```
    VerticalLeftAligned,
    /// Objects are aligned Verticaly and are horizotnaly centered with each others
    /// ```
    ///         ↓
    ///  +------------+
    ///  |      +     |
    ///  +------------+
    ///         ↓
    ///    +--------+
    ///    |    +   |
    ///    +--------+
    /// ```
    VerticalCenterAlgined,
    /// Objects are verticaly aligned by their right side
    /// ```
    ///           ↓
    ///  +---------+
    ///  |         |
    ///  +---------+
    ///            ↓
    ///     +------+
    ///     |      |
    ///     +------+
    /// ```
    VerticalRightAlgined,
    /// Objects are not aligned
    NonAligned,
}

/// Returns the alignement between two or more objects
///
/// Alignement can be auto implemented for any Struct which implements Coordinates and Shape traits
pub trait Alignement: Coordinates + Shape {
    /// This method returns the alignement of a set of objects
    fn alignement<X: Alignement>(&self, others: Vec<&X>) -> ObjectAlignement {
        //  +--------+  +---------+
        //  |........|  |.........|
        // →+--------+ →+---------+

        if others.iter().all(|elem| elem.y() == self.y()) {
            return ObjectAlignement::HorizontalAligned;
        }

        //              +-------------+
        //  +--------+  |             |
        // →|    +   | →|      +      |
        //  +--------+  |             |
        //              +-------------+

        if others
            .iter()
            .all(|elem| elem.height() / 2.0 + elem.y() == self.height() / 2.0 + self.y())
        {
            return ObjectAlignement::HorizontalCenterAligned;
        }

        //  ↓
        //  +---------+
        //  |         |
        //  +---------+
        //  ↓
        //  +------+
        //  |      |
        //  +------+

        if others.iter().all(|elem| elem.x() == self.x()) {
            return ObjectAlignement::VerticalLeftAligned;
        }

        //         ↓
        //  +------------+
        //  |      +     |
        //  +------------+
        //         ↓
        //    +--------+
        //    |    +   |
        //    +--------+

        if others
            .iter()
            .all(|elem| elem.width() / 2.0 + elem.x() == self.width() / 2.0 + self.x())
        {
            return ObjectAlignement::VerticalCenterAlgined;
        }

        //            ↓
        //  +---------+
        //  |         |
        //  +---------+
        //            ↓
        //     +------+
        //     |      |
        //     +------+

        if others
            .iter()
            .all(|elem| elem.width() + elem.x() == self.width() + self.x())
        {
            return ObjectAlignement::VerticalRightAlgined;
        }

        if others
            .iter()
            .all(|elem| elem.y() == self.y() && elem.x() == self.x())
        {
            return ObjectAlignement::Alinged;
        };

        ObjectAlignement::NonAligned
    }
}

/// Get the vertical and horizontal spacing of a set of objects
pub trait Spacing {
    /// This method returns a vector containing all the vertical spacing of a set of objects
    ///
    /// Vertical spacing for objects verticaly aligned are ignored
    fn vertical_spacing(&self) -> Vec<f32>;
    /// This method returns the most frequent (mode) vertical spacing of a set of objects
    fn mode_vertical_spacing(&self) -> Option<f32>;
    /// This method returns a vector containing all the horizontal spacing of a set of objects
    ///
    /// Horizontal spacing for objects horizontaly aligned are ignored
    fn horizontal_spacing(&self) -> Vec<f32>;
    /// This method returns the most frequent (mode) horizontal spacing of a set of objects
    fn mode_horizontal_spacing(&self) -> Option<f32>;
}

impl<'a, OBJECTSET, OBJECT> Spacing for OBJECTSET
where
    OBJECTSET: IntoIterator<Item = &'a OBJECT> + Clone,
    OBJECT: 'a + Coordinates + Alignement,
{
    /// Collect vertical spacing of a set of tokens
    fn vertical_spacing(&self) -> Vec<f32> {
        let tokens = self.clone().into_iter();
        let mut tkns = tokens.peekable();

        let mut vertical_spacing = Vec::new();

        while let Some(current_token) = tkns.next() {
            if let Some(next_token) = tkns.peek() {
                let spacing = current_token.base();
                let last_spacing = *vertical_spacing.last().unwrap_or(&0.0);

                let mut v = Vec::with_capacity(1);
                v.push(*next_token);

                // Only take into account tokens on different lines
                match current_token.alignement(v) {
                    ObjectAlignement::HorizontalAligned
                    | ObjectAlignement::HorizontalCenterAligned => (),
                    _ => {
                        if spacing > 0.0 && spacing > last_spacing {
                            vertical_spacing.push(next_token.y() - spacing);
                        }
                    }
                }
            }
        }

        // removing empty spaces
        vertical_spacing = vertical_spacing
            .into_iter()
            .filter(|spacing| *spacing > 0.0)
            .collect::<Vec<f32>>();

        vertical_spacing.shrink_to_fit();
        vertical_spacing
    }

    /// Returns the most frequent value of an iterator of vertical spacing
    fn mode_vertical_spacing(&self) -> Option<f32> {
        stats::mode(self.vertical_spacing().iter().map(|value| value.round()))
    }

    /// Collect vertical spacing of a set of tokens
    fn horizontal_spacing(&self) -> Vec<f32> {
        let tokens = self.clone().into_iter();
        let mut tkns = tokens.peekable();

        let mut horizontal_spacing = Vec::new();

        while let Some(current_token) = tkns.next() {
            if let Some(next_token) = tkns.peek() {
                let spacing = current_token.x() + current_token.width();
                let last_spacing = *horizontal_spacing.last().unwrap_or(&0.0);

                let mut v = Vec::with_capacity(1);
                v.push(*next_token);

                // Only take into account tokens on the same line
                if (ObjectAlignement::HorizontalAligned == current_token.alignement(v.clone())
                    || ObjectAlignement::HorizontalCenterAligned == current_token.alignement(v))
                    && spacing > 0.0
                    && spacing > last_spacing
                {
                    horizontal_spacing.push(next_token.x() - spacing);
                }
            }
        }

        // removing empty spaces
        horizontal_spacing = horizontal_spacing
            .into_iter()
            .filter(|spacing| *spacing > 0.0)
            .collect::<Vec<f32>>();

        horizontal_spacing.shrink_to_fit();
        horizontal_spacing
    }

    /// Returns the most frequent value of an iterator of horizontal spacing
    fn mode_horizontal_spacing(&self) -> Option<f32> {
        stats::mode(self.horizontal_spacing().iter().map(|value| value.round()))
    }
}
