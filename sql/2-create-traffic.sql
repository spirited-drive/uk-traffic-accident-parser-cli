-- ---------------------- --
-- ---- TRAFFIC DATA ---- --
-- ---------------------- --
CREATE SCHEMA IF NOT EXISTS traffic;

CREATE TABLE traffic.roads
(
    id                  SERIAL          NOT NULL        PRIMARY KEY,
    name                TEXT            NOT NULL        UNIQUE,
    category            TEXT            NULL,
    type                TEXT            NULL
);

CREATE TABLE traffic.count_points
(
    id                  SERIAL          NOT NULL        PRIMARY KEY,
    road_id             INTEGER         NOT NULL,
    vendor_id           INTEGER         NOT NULL,
    location            POINT           NOT NULL,

    FOREIGN KEY (road_id) REFERENCES traffic.roads(id)
        ON DELETE CASCADE
        ON UPDATE NO ACTION
);

CREATE TABLE traffic.counts
(
    id                  SERIAL          NOT NULL        PRIMARY KEY,
    count_point_id      INTEGER         NOT NULL,
    road_id             INTEGER         NOT NULL,
    vendor_id           INTEGER         NOT NULL        UNIQUE,
    date                TIMESTAMP       NOT NULL,
    hour                INTEGER         NOT NULL,
    direction           TEXT            NOT NULL,
    location            POINT           NOT NULL,
    bicycles            INTEGER         NOT NULL,
    motorcycles         INTEGER         NOT NULL,
    cars                INTEGER         NOT NULL,
    buses               INTEGER         NOT NULL,
    lgvs                INTEGER         NOT NULL,
    hgvs                INTEGER         NOT NULL,

    FOREIGN KEY (count_point_id) REFERENCES traffic.count_points(id)
        ON DELETE CASCADE
        ON UPDATE NO ACTION,

    FOREIGN KEY (road_id) REFERENCES traffic.roads(id)
        ON DELETE CASCADE
        ON UPDATE NO ACTION
);