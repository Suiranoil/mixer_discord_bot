-- DROP TABLE IF EXISTS players_test;
-- DROP FUNCTION IF EXISTS rating_not_null;
-- DROP TYPE rating;
-- DROP TYPE IF EXISTS role;


-- CREATE TYPE rating AS (
-- 	rating REAL,
-- 	rd REAL,
-- 	volatility REAL
-- );

-- CREATE OR REPLACE FUNCTION rating_not_null(r rating) RETURNS BOOLEAN AS $$
-- BEGIN
--     RETURN (r.rating IS NOT NULL) AND (r.rd IS NOT NULL) AND (r.volatility IS NOT NULL);
-- END;
-- $$ LANGUAGE plpgsql;

-- CREATE TABLE players_test (
-- 	id SERIAL PRIMARY KEY,
-- 	discord_id BIGINT NOT NULL,
-- 	bn_name TEXT DEFAULT '',
-- 	bn_tag TEXT DEFAULT '',

-- 	tank_rating rating NOT NULL DEFAULT (2500.0, 300.0, 0.06),
-- 	dps_rating rating NOT NULL DEFAULT (2500.0, 300.0, 0.06),
-- 	support_rating rating NOT NULL DEFAULT (2500.0, 300.0, 0.06),

-- 	flex BOOLEAN NOT NULL DEFAULT true,
-- 	primary_role role,
-- 	secondary_role role,
-- 	tertiary_role role,
-- 	UNIQUE (discord_id),
-- 	CHECK (
-- 		rating_not_null(tank_rating) AND
-- 		rating_not_null(dps_rating) AND
-- 		rating_not_null(support_rating)
-- 	)                                                           
-- );

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'role') THEN
         CREATE TYPE role AS ENUM ('tank', 'dps', 'support');
    END IF;
END$$;

CREATE TABLE IF NOT EXISTS players (
   id SERIAL PRIMARY KEY,
   discord_id BIGINT NOT NULL,
   bn_name TEXT,
   bn_tag TEXT,
   last_played TIMESTAMP,
   tank_rating REAL NOT NULL DEFAULT 2500.0,
   tank_rd REAL NOT NULL DEFAULT 300,
   tank_volatility REAL NOT NULL DEFAULT 0.06,
   dps_rating REAL NOT NULL DEFAULT 2500.0,
   dps_rd REAL NOT NULL DEFAULT 300,
   dps_volatility REAL NOT NULL DEFAULT 0.06,
   support_rating REAL NOT NULL DEFAULT 2500.0,
   support_rd REAL NOT NULL DEFAULT 300,
   support_volatility REAL NOT NULL DEFAULT 0.06,
   flex BOOLEAN NOT NULL DEFAULT true,
   primary_role role,
   secondary_role role,
   tertiary_role role,
   UNIQUE (discord_id)
);

CREATE TABLE IF NOT EXISTS guilds (
   id SERIAL PRIMARY KEY,
   guild_id BIGINT NOT NULL,
   verified BOOLEAN NOT NULL DEFAULT false,

   UNIQUE (guild_id)
);

CREATE TABLE IF NOT EXISTS lobbies (
   id SERIAL PRIMARY KEY,
   guild_id BIGINT NOT NULL,
   main_voice_id BIGINT NOT NULL,
   red_team_voice_id BIGINT NOT NULL,
   blue_team_voice_id BIGINT NOT NULL
);

CREATE INDEX IF NOT EXISTS players_discord_id_idx ON players (discord_id);
CREATE INDEX IF NOT EXISTS lobbies_guild_id_idx ON lobbies (guild_id);
CREATE INDEX IF NOT EXISTS lobbies_main_voice_id_idx ON lobbies (main_voice_id);
CREATE INDEX IF NOT EXISTS lobbies_red_team_voice_id_idx ON lobbies (red_team_voice_id);
CREATE INDEX IF NOT EXISTS lobbies_blue_team_voice_id_idx ON lobbies (blue_team_voice_id);
CREATE INDEX IF NOT EXISTS guilds_guild_id_idx ON guilds (guild_id);