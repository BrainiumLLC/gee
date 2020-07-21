use crate::{Angle, Vector};
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

impl Cardinal {
    pub fn angle<T: en::Float>(self) -> Angle<T> {
        Direction::from(self).angle()
    }

    pub fn unit_vector<T: en::Float>(self) -> Vector<T> {
        Direction::from(self).unit_vector()
    }
}

impl<T: en::Float> Into<Angle<T>> for Cardinal {
    fn into(self) -> Angle<T> {
        self.angle()
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

impl Direction {
    pub fn angle<T: en::Float>(self) -> Angle<T> {
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

    pub fn unit_vector<T: en::Float>(self) -> Vector<T> {
        self.angle().unit_vector()
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

impl<T: en::Float> Into<Angle<T>> for Direction {
    fn into(self) -> Angle<T> {
        self.angle()
    }
}

impl From<Cardinal> for Direction {
    fn from(cardinal: Cardinal) -> Self {
        match cardinal {
            Cardinal::North => Direction::North,
            Cardinal::South => Direction::South,
            Cardinal::East => Direction::East,
            Cardinal::West => Direction::West,
        }
    }
}

impl TryFrom<Direction> for Cardinal {
    type Error = ();
    fn try_from(direction: Direction) -> Result<Self, Self::Error> {
        match direction {
            Direction::North => Ok(Cardinal::North),
            Direction::East => Ok(Cardinal::East),
            Direction::South => Ok(Cardinal::South),
            Direction::West => Ok(Cardinal::West),
            _ => Err(()),
        }
    }
}
