use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Guilds::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Guilds::Id)
                            .integer()
                            .not_null()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(Guilds::GuildId)
                            .big_unsigned()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Guilds::Verified)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .primary_key(Index::create().name("guilds_pkey").col(Guilds::Id))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("guilds_guild_id_idx")
                    .table(Guilds::Table)
                    .col(Guilds::GuildId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .table(Guilds::Table)
                    .name("guilds_guild_id_idx")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Guilds::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum Guilds {
    Table,
    Id,
    GuildId,
    Verified,
}
