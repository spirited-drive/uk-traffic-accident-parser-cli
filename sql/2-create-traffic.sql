-- ---------------------- --
-- ---- TRAFFIC DATA ---- --
-- ---------------------- --
CREATE SCHEMA IF NOT EXISTS traffic;


-- ---- ROAD CATEGORIES ---- --
-- ------------------------- --
CREATE TABLE IF NOT EXISTS traffic.road_categories
(
    id                  SERIAL          NOT NULL        PRIMARY KEY,
    name                TEXT            NOT NULL        UNIQUE,
    description         TEXT            NOT NULL,
    is_major            BOOL            NOT NULL        DEFAULT false
);

INSERT INTO traffic.road_categories(name, description, is_major)
VALUES('PM', 'M or Class A Principal Motorway', true)
ON CONFLICT (name) DO NOTHING;

INSERT INTO traffic.road_categories(name, description, is_major)
VALUES('PA', 'Class A Principal road', true)
ON CONFLICT (name) DO NOTHING;

INSERT INTO traffic.road_categories(name, description, is_major)
VALUES('TM', 'M or Class A Trunk Motorway', true)
ON CONFLICT (name) DO NOTHING;

INSERT INTO traffic.road_categories(name, description, is_major)
VALUES('TA', 'Class A Trunk road', true)
ON CONFLICT (name) DO NOTHING;

INSERT INTO traffic.road_categories(name, description, is_major)
VALUES('M', 'Minor road', false)
ON CONFLICT (name) DO NOTHING;

INSERT INTO traffic.road_categories(name, description, is_major)
VALUES('MB', 'Class B road', false)
ON CONFLICT (name) DO NOTHING;

INSERT INTO traffic.road_categories(name, description, is_major)
VALUES('MCU', 'Class C road or Unclassified road', false)
ON CONFLICT (name) DO NOTHING;



-- ---- REGIONS ---- --
-- ----------------- --
CREATE TABLE IF NOT EXISTS traffic.regions 
(
    id                  INTEGER         NOT NULL        PRIMARY KEY,
    name                TEXT            NOT NULL        UNIQUE,
    ons_code            TEXT            NOT NULL        UNIQUE
);



-- ---- LOCAL AUTHORITIES ---- --
-- --------------------------- --
CREATE TABLE IF NOT EXISTS traffic.local_authorities
(
    id                  INTEGER         NOT NULL        PRIMARY KEY,
    region_id           INTEGER         NOT NULL,
    name                TEXT            NOT NULL,
    ons_code            TEXT            NOT NULL        UNIQUE,

    FOREIGN KEY (region_id) REFERENCES traffic.regions(id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION
);

CREATE INDEX IF NOT EXISTS local_authorities_region_id_idx
ON traffic.local_authorities (region_id);



-- ---- COUNT POINTS ---- --
-- ---------------------- --
CREATE TABLE IF NOT EXISTS traffic.count_points
(
    id                  INTEGER         NOT NULL        PRIMARY KEY,
    local_authority_id  INTEGER         NOT NULL,
    road_category_id    INTEGER         NOT NULL,
    road_name           TEXT            NOT NULL,
    location            POINT           NOT NULL,
    
    FOREIGN KEY (local_authority_id) REFERENCES traffic.local_authorities(id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION,

    FOREIGN KEY (road_category_id) REFERENCES traffic.road_categories(id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION
);

CREATE INDEX IF NOT EXISTS count_points_local_authority_id_idx
ON traffic.count_points (local_authority_id);

CREATE INDEX IF NOT EXISTS count_points_road_category_id_idx
ON traffic.count_points (road_category_id);



-- ---- COUNTS ---- --
-- ---------------- --
CREATE TABLE IF NOT EXISTS traffic.counts
(
    id                  SERIAL          NOT NULL        PRIMARY KEY,
    count_point_id      INTEGER         NOT NULL,
    date                DATE            NOT NULL,
    hour                SMALLINT        NOT NULL        CHECK (hour BETWEEN 0 AND 23),
    direction           CHAR(1)         NOT NULL        CHECK (direction IN ('N', 'E', 'S', 'W', 'C', 'J')),
    bicycles            INTEGER         NOT NULL        CHECK (bicycles >= 0),
    motorcycles         INTEGER         NOT NULL        CHECK (motorcycles >= 0),
    cars                INTEGER         NOT NULL        CHECK (cars >= 0),
    buses               INTEGER         NOT NULL        CHECK (buses >= 0),
    lgvs                INTEGER         NOT NULL        CHECK (lgvs >= 0),
    hgvs                INTEGER         NOT NULL        CHECK (hgvs >= 0),

    FOREIGN KEY (count_point_id) REFERENCES traffic.count_points(id)
        ON DELETE CASCADE
        ON UPDATE NO ACTION,

    CONSTRAINT counts_unique_observation
        UNIQUE (count_point_id, date, hour, direction)
);

CREATE INDEX IF NOT EXISTS counts_count_point_id_idx
ON traffic.counts (count_point_id);