use crate::Angle;
use num_traits::{Float, FloatConst};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, ops::Neg};
use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Cardinal {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Direction {
    North,
    Northeast,
    East,
    Southeast,
    South,
    Southwest,
    West,
    Northwest,
}

impl Cardinal {
    pub fn angle<T: Float + FloatConst>(self) -> Angle<T> {
        use Cardinal::*;
        match self {
            North => Angle::FRAC_PI_2(),
            South => Angle::FRAC_3PI_2(),
            East => Angle::ZERO(),
            West => Angle::PI(),
        }
    }
}

impl Direction {
    pub fn angle<T: Float + FloatConst>(self) -> Angle<T> {
        use Direction::*;
        match self {
            North => Angle::FRAC_PI_2(),
            South => Angle::FRAC_3PI_2(),
            East => Angle::ZERO(),
            West => Angle::PI(),
            Northeast => Angle::FRAC_PI_4(),
            Southeast => Angle::FRAC_7PI_4(),
            Southwest => Angle::FRAC_5PI_4(),
            Northwest => Angle::FRAC_3PI_4(),
        }
    }
}

impl Neg for Cardinal {
    type Output = Self;
    fn neg(self) -> Self::Output {
        use Cardinal::*;
        match self {
            North => South,
            South => North,
            East => West,
            West => East,
        }
    }
}

impl Neg for Direction {
    type Output = Self;
    fn neg(self) -> Self::Output {
        use Direction::*;
        match self {
            North => South,
            Northeast => Southwest,
            East => West,
            Southeast => Northwest,
            South => North,
            Southwest => Northeast,
            West => East,
            Northwest => Southeast,
        }
    }
}

impl From<Cardinal> for Direction {
    fn from(cardinal: Cardinal) -> Self {
        match cardinal {
            Cardinal::North => Direction::South,
            Cardinal::South => Direction::North,
            Cardinal::East => Direction::West,
            Cardinal::West => Direction::East,
        }
    }
}

impl TryFrom<Direction> for Cardinal {
    type Error = ();
    fn try_from(direction: Direction) -> Result<Self, Self::Error> {
        match direction {
            Direction::North => Ok(Cardinal::South),
            Direction::East => Ok(Cardinal::West),
            Direction::South => Ok(Cardinal::North),
            Direction::West => Ok(Cardinal::East),
            _ => Err(()),
        }
    }
}
