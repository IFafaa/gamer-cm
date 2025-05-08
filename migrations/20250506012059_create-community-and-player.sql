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
        community_id INT REFERENCES communities (id) ON DELETE CASCADE,
        created_at TIMESTAMP NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMP NOT NULL DEFAULT NOW (),
        enabled BOOLEAN NOT NULL DEFAULT TRUE
    );