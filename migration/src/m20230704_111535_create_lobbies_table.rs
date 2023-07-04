use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Lobbies::Table)
                    .col(
                        ColumnDef::new(Lobbies::Id)
                            .integer()
                            .not_null()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Lobbies::GuildId).big_unsigned().not_null())
                    .col(
                        ColumnDef::new(Lobbies::MainVoiceId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Lobbies::RedTeamVoiceId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Lobbies::BlueTeamVoiceId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .primary_key(Index::create().name("lobbies_pkey").col(Lobbies::Id))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("lobbies_guild_id_idx")
                    .table(Lobbies::Table)
                    .col(Lobbies::GuildId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("lobbies_main_voice_id_idx")
                    .table(Lobbies::Table)
                    .col(Lobbies::MainVoiceId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("lobbies_red_team_voice_id_idx")
                    .table(Lobbies::Table)
                    .col(Lobbies::RedTeamVoiceId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("lobbies_blue_team_voice_id_idx")
                    .table(Lobbies::Table)
                    .col(Lobbies::BlueTeamVoiceId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("lobbies_guild_id_idx")
                    .table(Lobbies::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("lobbies_main_voice_id_idx")
                    .table(Lobbies::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("lobbies_red_team_voice_id_idx")
                    .table(Lobbies::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("lobbies_blue_team_voice_id_idx")
                    .table(Lobbies::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Lobbies::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum Lobbies {
    Table,
    Id,
    GuildId,
    MainVoiceId,
    RedTeamVoiceId,
    BlueTeamVoiceId,
}
