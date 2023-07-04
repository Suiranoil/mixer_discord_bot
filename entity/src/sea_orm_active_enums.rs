use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "role")]
pub enum Role {
    #[sea_orm(string_value = "tank")]
    Tank,
    #[sea_orm(string_value = "dps")]
    Dps,
    #[sea_orm(string_value = "support")]
    Support,
}

impl TryFrom<&str> for Role {
    type Error = String;

    fn try_from(val: &str) -> Result<Self, Self::Error> {
        match val {
            "tank" => Ok(Role::Tank),
            "dps" => Ok(Role::Dps),
            "support" => Ok(Role::Support),
            _ => Err(format!("Unknown enum variant '{}'", val)),
        }
    }
}

impl From<&Role> for i32 {
    fn from(val: &Role) -> Self {
        match val {
            Role::Tank => 0,
            Role::Dps => 1,
            Role::Support => 2,
        }
    }
}
