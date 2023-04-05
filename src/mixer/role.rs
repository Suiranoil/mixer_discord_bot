use std::hash::Hash;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Role {
    Tank,
    Dps,
    Support
}

impl FromStr for Role {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tank" => Ok(Role::Tank),
            "dps" => Ok(Role::Dps),
            "support" => Ok(Role::Support),
            _ => Err(())
        }
    }
}

impl From<i32> for Role {
    fn from(i: i32) -> Self {
        match i {
            0 => Role::Tank,
            1 => Role::Dps,
            2 => Role::Support,
            _ => panic!("Invalid role number")
        }
    }
}

impl PartialEq<i32> for Role {
    fn eq(&self, other: &i32) -> bool {
        match self {
            Role::Tank => *other == 0,
            Role::Dps => *other == 1,
            Role::Support => *other == 2
        }
    }
}

impl Role {
    pub fn iter() -> impl Iterator<Item = Role> {
        vec![Role::Tank, Role::Dps, Role::Support].into_iter()
    }

    pub fn option_to_i32(role: Option<Role>) -> i32 {
        match role {
            Some(role) => role as i32,
            None => -1
        }
    }
}