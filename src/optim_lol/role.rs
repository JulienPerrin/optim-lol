use crate::*;
use std::cmp::Eq;
use std::hash::Hash;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Role {
    TOP = 1,
    JUNGLE = 2,
    MID = 3,
    BOTTOM = 4,
    SUPPORT = 5,
}

impl Role {
    pub const fn nombre_roles() -> usize {
        5
    }

    pub fn from_u8(value: u8) -> Role {
        match value {
            1 => TOP,
            2 => JUNGLE,
            3 => MID,
            4 => BOTTOM,
            5 => SUPPORT,
            _ => panic!("Unknown value for Role: {}", value),
        }
    }
}
