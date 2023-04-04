CREATE TABLE IF NOT EXISTS players (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    discord_id INTEGER NOT NULL,
    bn_name TEXT,
    bn_tag TEXT,
    tank REAL NOT NULL DEFAULT 2500.0,
    dps REAL NOT NULL DEFAULT 2500.0,
    support REAL NOT NULL DEFAULT 2500.0,
    flex INTEGER NOT NULL DEFAULT true,
    primary_role INTEGER NOT NULL DEFAULT -1,
    secondary_role INTEGER NOT NULL DEFAULT -1,
    tertiary_role INTEGER NOT NULL DEFAULT -1,
    UNIQUE (discord_id)
);

CREATE TABLE IF NOT EXISTS lobbies (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    guild_id INTEGER NOT NULL,
    main_voice_id INTEGER NOT NULL,
    red_team_voice_id INTEGER NOT NULL,
    blue_team_voice_id INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS players_discord_id_idx ON players (discord_id);
CREATE INDEX IF NOT EXISTS lobbies_guild_id_idx ON lobbies (guild_id);
CREATE INDEX IF NOT EXISTS lobbies_main_voice_id_idx ON lobbies (main_voice_id);
CREATE INDEX IF NOT EXISTS lobbies_red_team_voice_id_idx ON lobbies (red_team_voice_id);
CREATE INDEX IF NOT EXISTS lobbies_blue_team_voice_id_idx ON lobbies (blue_team_voice_id);
