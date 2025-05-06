CREATE TABLE
    communities (
        id SERIAL PRIMARY KEY,
        name VARCHAR(255) NOT NULL UNIQUE,
        created_at TIMESTAMP NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMP NOT NULL DEFAULT NOW ()
    );

CREATE TABLE
    players (
        id SERIAL PRIMARY KEY,
        nickname VARCHAR(255) NOT NULL UNIQUE,
        community_id INT REFERENCES communities (id) ON DELETE CASCADE,
        created_at TIMESTAMP NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMP NOT NULL DEFAULT NOW ()
    );