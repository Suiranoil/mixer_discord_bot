use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Players::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Players::Id)
                            .integer()
                            .not_null()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(Players::DiscordId)
                            .big_unsigned()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Players::BnName).string())
                    .col(ColumnDef::new(Players::BnTag).string())
                    .col(ColumnDef::new(Players::LastPlayed).timestamp())
                    .col(
                        ColumnDef::new(Players::TankRating)
                            .float()
                            .not_null()
                            .default(2500),
                    )
                    .col(
                        ColumnDef::new(Players::TankRd)
                            .float()
                            .not_null()
                            .default(300),
                    )
                    .col(
                        ColumnDef::new(Players::TankVolatility)
                            .float()
                            .not_null()
                            .default(0.06),
                    )
                    .col(
                        ColumnDef::new(Players::DpsRating)
                            .float()
                            .not_null()
                            .default(2500),
                    )
                    .col(
                        ColumnDef::new(Players::DpsRd)
                            .float()
                            .not_null()
                            .default(300),
                    )
                    .col(
                        ColumnDef::new(Players::DpsVolatility)
                            .float()
                            .not_null()
                            .default(0.06),
                    )
                    .col(
                        ColumnDef::new(Players::SupportRating)
                            .float()
                            .not_null()
                            .default(2500),
                    )
                    .col(
                        ColumnDef::new(Players::SupportRd)
                            .float()
                            .not_null()
                            .default(300),
                    )
                    .col(
                        ColumnDef::new(Players::SupportVolatility)
                            .float()
                            .not_null()
                            .default(0.06),
                    )
                    .col(
                        ColumnDef::new(Players::Flex)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(ColumnDef::new(Players::PrimaryRole).custom(Role))
                    .col(ColumnDef::new(Players::SecondaryRole).custom(Role))
                    .col(ColumnDef::new(Players::TertiaryRole).custom(Role))
                    .primary_key(Index::create().name("players_pkey").col(Players::Id))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("players_discord_id_idx")
                    .table(Players::Table)
                    .col(Players::DiscordId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name("players_discord_id_idx").to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Players::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum Players {
    Table,
    Id,
    DiscordId,
    BnName,
    BnTag,
    LastPlayed,
    TankRating,
    TankRd,
    TankVolatility,
    DpsRating,
    DpsRd,
    DpsVolatility,
    SupportRating,
    SupportRd,
    SupportVolatility,
    Flex,
    PrimaryRole,
    SecondaryRole,
    TertiaryRole,
}

#[derive(Iden)]
struct Role;
