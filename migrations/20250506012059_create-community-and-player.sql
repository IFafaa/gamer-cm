CREATE TABLE
    communities (
        id SERIAL PRIMARY KEY,
        name VARCHAR(255) NOT NULL,
        created_at TIMESTAMP NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMP NOT NULL DEFAULT NOW (),
        enabled BOOLEAN NOT NULL DEFAULT TRUE
    );

CREATE TABLE
    players (
        id SERIAL PRIMARY KEY,
        nickname VARCHAR(255) NOT NULL,
        community_id INT REFERENCES communities (id) ON DELETE CASCADE NOT NULL,
        created_at TIMESTAMP NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMP NOT NULL DEFAULT NOW (),
        enabled BOOLEAN NOT NULL DEFAULT TRUE
    );

CREATE TABLE
    teams (
        id SERIAL PRIMARY KEY,
        name VARCHAR(255) NOT NULL,
        community_id INT REFERENCES communities (id) ON DELETE CASCADE NOT NULL,
        created_at TIMESTAMP NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMP NOT NULL DEFAULT NOW (),
        enabled BOOLEAN NOT NULL DEFAULT TRUE
    );

CREATE TABLE
    team_players (
        player_id INT REFERENCES players (id) ON DELETE CASCADE NOT NULL,
        team_id INT REFERENCES teams (id) ON DELETE CASCADE NOT NULL,
        created_at TIMESTAMP NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMP NOT NULL DEFAULT NOW (),
        enabled BOOLEAN NOT NULL DEFAULT TRUE,
        PRIMARY KEY (player_id, team_id)
    );

CREATE TABLE
    parties (
        id SERIAL PRIMARY KEY,
        game_name VARCHAR(255),
        team_winner_id INT REFERENCES teams (id) ON DELETE SET NULL,
        community_id INT REFERENCES communities (id) ON DELETE CASCADE NOT NULL,
        created_at TIMESTAMP NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMP NOT NULL DEFAULT NOW (),
        enabled BOOLEAN NOT NULL DEFAULT TRUE
    );

CREATE TABLE
    party_teams (
        team_id INT REFERENCES teams (id) ON DELETE CASCADE NOT NULL,
        party_id INT REFERENCES parties (id) ON DELETE CASCADE NOT NULL,
        created_at TIMESTAMP NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMP NOT NULL DEFAULT NOW (),
        enabled BOOLEAN NOT NULL DEFAULT TRUE,
        PRIMARY KEY (team_id, party_id)
    );

CREATE INDEX idx_players_community_id ON players (community_id);

CREATE INDEX idx_teams_community_id ON teams (community_id);

CREATE INDEX idx_team_players_team_id ON team_players (team_id);

CREATE INDEX idx_party_teams_party_id ON party_teams (party_id);

CREATE INDEX idx_parties_community_id ON parties (community_id);