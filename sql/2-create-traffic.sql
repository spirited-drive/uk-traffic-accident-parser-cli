-- ---------------------- --
-- ---- TRAFFIC DATA ---- --
-- ---------------------- --
CREATE SCHEMA IF NOT EXISTS traffic;

CREATE TABLE traffic.road_categories
(
    id                  SERIAL          NOT NULL        PRIMARY KEY,
    name                TEXT            NOT NULL        UNIQUE,
    description         TEXT            NOT NULL,
    is_major            BOOL            NOT NULL        DEFAULT false
);

INSERT INTO traffic.road_categories(name, description, is_major)
VALUES('PM', 'M or Class A Principal Motorway', true);

INSERT INTO traffic.road_categories(name, description, is_major)
VALUES('PA', 'Class A Principal road', true);

INSERT INTO traffic.road_categories(name, description, is_major)
VALUES('TM', 'M or Class A Trunk Motorway', true);

INSERT INTO traffic.road_categories(name, description, is_major)
VALUES('TA', 'Class A Trunk road', true);

INSERT INTO traffic.road_categories(name, description, is_major)
VALUES('M', 'Minor road', false);

INSERT INTO traffic.road_categories(name, description, is_major)
VALUES('MB', 'Class B road', false);

INSERT INTO traffic.road_categories(name, description, is_major)
VALUES('MCU', 'Class C road or Unclassified road', false);

CREATE TABLE traffic.regions 
(
    id                  SERIAL          NOT NULL        PRIMARY KEY,
    name                TEXT            NOT NULL        UNIQUE,
    ons_code            TEXT            NOT NULL        UNIQUE
);

CREATE TABLE traffic.local_authorities
(
    id                  SERIAL          NOT NULL        PRIMARY KEY,
    region_id           INTEGER         NOT NULL,
    name                TEXT            NOT NULL,
    ons_code            TEXT            NOT NULL        UNIQUE,

    FOREIGN KEY (region_id) REFERENCES traffic.regions(id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION
);

CREATE TABLE traffic.count_points
(
    id                  SERIAL          NOT NULL        PRIMARY KEY,
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

CREATE TABLE traffic.counts
(
    id                  SERIAL          NOT NULL        PRIMARY KEY,
    count_point_id      INTEGER         NOT NULL,
    date                DATE            NOT NULL,
    hour                SMALLINT        NOT NULL,
    direction           CHAR(1)         NOT NULL,
    bicycles            INTEGER         NOT NULL,
    motorcycles         INTEGER         NOT NULL,
    cars                INTEGER         NOT NULL,
    buses               INTEGER         NOT NULL,
    lgvs                INTEGER         NOT NULL,
    hgvs                INTEGER         NOT NULL,

    FOREIGN KEY (count_point_id) REFERENCES traffic.count_points(id)
        ON DELETE CASCADE
        ON UPDATE NO ACTION
);