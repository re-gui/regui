use std::{any::Any, str::FromStr};

mod color; pub use color::*;

pub struct Style {
    declarations: Vec<CssDeclaration>,
}

impl Style {
    pub fn new() -> Self {
        Self {
            declarations: Vec::new(),
        }
    }
}

pub struct CssDeclaration {
    pub selector: String,
    pub properties: Box<dyn Any>,
}

pub enum CssPropertyValue {

}

pub enum CssDomLenght {

}

// https://www.w3schools.com/cssref/css_units.php
#[allow(non_camel_case_types)]
pub enum CssAbsoluteLenghtUnits {
    /// centimeters
    cm(f32),

    /// millimeters
    mm(f32),

    /// inches
    inches(f32),

    /// pixels
    px(f32),

    /// points (1pt = 1/72th of 1in = ~0.353mm)
    pt(f32),

    /// picas (1pc = 12 pt)
    pc(f32),
}

#[allow(non_camel_case_types)]
pub enum CssRelativeLengthUnits {
    /// relative to the font-size of the element (2em means 2 times the size of the current font)
    em(f32),

    /// relative to the x-height of the current font (rarely used)
    ex(f32),

    /// relative to width of the "0" (zero)
    ch(f32),

    /// relative to the font-size of the root element
    rem(f32),

    /// relative to 1% of the width of the viewport*
    vw(f32),

    /// relative to 1% of the height of the viewport*
    vh(f32),

    /// relative to 1% of viewport's* smaller dimension
    vmin(f32),

    /// relative to 1% of viewport's* larger dimension
    vmax(f32),

    /// 1% of the parent container's size
    percent(f32),
}

pub mod css_units {
    pub use super::CssAbsoluteLenghtUnits::*;
    pub use super::CssRelativeLengthUnits::*;
}

use css_units::*;

pub enum CssLengthUnits {
    Absolute(CssAbsoluteLenghtUnits),
    Relative(CssRelativeLengthUnits),
}

impl From<CssAbsoluteLenghtUnits> for CssLengthUnits {
    fn from(unit: CssAbsoluteLenghtUnits) -> Self {
        Self::Absolute(unit)
    }
}

impl From<CssRelativeLengthUnits> for CssLengthUnits {
    fn from(unit: CssRelativeLengthUnits) -> Self {
        Self::Relative(unit)
    }
}

impl From<f32> for CssLengthUnits {
    fn from(value: f32) -> Self {
        px(value).into()
    }
}

pub enum ValueWithUnitsParseError {
    WrongNumberOfPieces,
    InvalidNumberValue,
    InvalidUnit,
}

impl FromStr for CssAbsoluteLenghtUnits {
    type Err = ValueWithUnitsParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (n, u) = {
            let mut pieces = s.split_whitespace();
            let n = pieces.next().ok_or(ValueWithUnitsParseError::WrongNumberOfPieces)?;
            let u = pieces.next().ok_or(ValueWithUnitsParseError::WrongNumberOfPieces)?;
            if pieces.next().is_some() {
                return Err(ValueWithUnitsParseError::WrongNumberOfPieces);
            }
            (n, u)
        };

        let n = n.parse::<f32>().map_err(|_| ValueWithUnitsParseError::InvalidNumberValue)?;
        if !n.is_finite() {
            return Err(ValueWithUnitsParseError::InvalidNumberValue);
        }

        match u {
            "cm" => Ok(cm(n)),
            "mm" => Ok(mm(n)),
            "in" => Ok(inches(n)),
            "px" => Ok(px(n)),
            "pt" => Ok(pt(n)),
            "pc" => Ok(pc(n)),
            _ => Err(ValueWithUnitsParseError::InvalidUnit),
        }
    }
}

impl FromStr for CssRelativeLengthUnits {
    type Err = ValueWithUnitsParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (n, u) = {
            let mut pieces = s.split_whitespace();
            let n = pieces.next().ok_or(ValueWithUnitsParseError::WrongNumberOfPieces)?;
            let u = pieces.next().ok_or(ValueWithUnitsParseError::WrongNumberOfPieces)?;
            if pieces.next().is_some() {
                return Err(ValueWithUnitsParseError::WrongNumberOfPieces);
            }
            (n, u)
        };

        let n = n.parse::<f32>().map_err(|_| ValueWithUnitsParseError::InvalidNumberValue)?;
        if !n.is_finite() {
            return Err(ValueWithUnitsParseError::InvalidNumberValue);
        }

        match u {
            "em" => Ok(em(n)),
            "ex" => Ok(ex(n)),
            "ch" => Ok(ch(n)),
            "rem" => Ok(rem(n)),
            "vw" => Ok(vw(n)),
            "vh" => Ok(vh(n)),
            "vmin" => Ok(vmin(n)),
            "vmax" => Ok(vmax(n)),
            "%" => Ok(percent(n)),
            _ => Err(ValueWithUnitsParseError::InvalidUnit),
        }
    }
}