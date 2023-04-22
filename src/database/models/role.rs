use sea_orm::{DeriveActiveEnum, EnumIter};

#[derive(EnumIter, DeriveActiveEnum, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "role")]
pub enum Role {
    #[sea_orm(string_value = "tank")]
    Tank,
    #[sea_orm(string_value = "dps")]
    Dps,
    #[sea_orm(string_value = "support")]
    Support,
}

impl From<Role> for String {
    fn from(role: Role) -> Self {
        match role {
            Role::Tank => "tank".to_string(),
            Role::Dps => "dps".to_string(),
            Role::Support => "support".to_string(),
        }
    }
}

impl TryFrom<&str> for Role {
    type Error = ();

    fn try_from(role: &str) -> Result<Self, Self::Error> {
        match role {
            "tank" => Ok(Role::Tank),
            "dps" => Ok(Role::Dps),
            "support" => Ok(Role::Support),
            _ => Err(()),
        }
    }
}

impl From<Role> for i32 {
    fn from(role: Role) -> Self {
        match role {
            Role::Tank => 0,
            Role::Dps => 1,
            Role::Support => 2,
        }
    }
}
