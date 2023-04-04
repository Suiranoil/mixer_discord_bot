use std::hash::{Hash, Hasher};
use std::str::FromStr;

#[derive(Debug, Ord, PartialOrd, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Role {
    Tank,
    Dps,
    Support,
    None
}


impl PartialEq<i32> for Role {
    fn eq(&self, other: &i32) -> bool {
        match self {
            Role::Tank => *other == 0,
            Role::Dps => *other == 1,
            Role::Support => *other == 2,
            Role::None => *other == -1
        }
    }
}

impl PartialEq<Role> for i32 {
    fn eq(&self, other: &Role) -> bool {
        match other {
            Role::Tank => *self == 0,
            Role::Dps => *self == 1,
            Role::Support => *self == 2,
            Role::None => *self == -1
        }
    }
}

impl From<&Role> for i32 {
    fn from(role: &Role) -> Self {
        match role {
            Role::Tank => 0,
            Role::Dps => 1,
            Role::Support => 2,
            Role::None => -1
        }
    }
}

impl From<i32> for Role {
    fn from(i: i32) -> Self {
        match i {
            0 => Role::Tank,
            1 => Role::Dps,
            2 => Role::Support,
            _ => Role::None
        }
    }
}

impl Into<i32> for Role {
    fn into(self) -> i32 {
        match self {
            Role::Tank => 0,
            Role::Dps => 1,
            Role::Support => 2,
            Role::None => -1
        }
    }
}

impl FromStr for Role {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tank" => Ok(Role::Tank),
            "dps" => Ok(Role::Dps),
            "support" => Ok(Role::Support),
            "none" => Ok(Role::None),
            _ => Ok(Role::None)
        }
    }
}

