-- ------------------------ --
-- ---- ACCIDENTS DATA ---- --
-- ------------------------ --
CREATE SCHEMA IF NOT EXISTS accidents;


-- ---- POLICE FORCES ---- --
-- ----------------------- --
CREATE TABLE IF NOT EXISTS accidents.police_forces
(
    id                  INTEGER         NOT NULL        PRIMARY KEY,
    name                TEXT            NOT NULL        UNIQUE
);

INSERT INTO accidents.police_forces (id, name)
VALUES 
    (1, 'Metropolitan Police'),
    (3, 'Cumbria'),
    (4, 'Lancashire'),
    (5, 'Merseyside'),
    (6, 'Greater Manchester'),
    (7, 'Cheshire'),
    (10, 'Northumbria'),
    (11, 'Durham'),
    (12, 'North Yorkshire'),
    (13, 'West Yorkshire'),
    (14, 'South Yorkshire'),
    (16, 'Humberside'),
    (17, 'Cleveland'),
    (20, 'West Midlands'),
    (21, 'Staffordshire'),
    (22, 'West Mercia'),
    (23, 'Warwickshire'),
    (30, 'Derbyshire'),
    (31, 'Nottinghamshire'),
    (32, 'Lincolnshire'),
    (33, 'Leicestershire'),
    (34, 'Northamptonshire'),
    (35, 'Cambridgeshire'),
    (36, 'Norfolk'),
    (37, 'Suffolk'),
    (40, 'Bedfordshire'),
    (41, 'Hertfordshire'),
    (42, 'Essex'),
    (43, 'Thames Valley'),
    (44, 'Hampshire'),
    (45, 'Surrey'),
    (46, 'Kent'),
    (47, 'Sussex'),
    (48, 'City of London'),
    (50, 'Devon and Cornwall'),
    (52, 'Avon and Somerset'),
    (53, 'Gloucestershire'),
    (54, 'Wiltshire'),
    (55, 'Dorset'),
    (60, 'North Wales'),
    (61, 'Gwent'),
    (62, 'South Wales'),
    (63, 'Dyfed-Powys'),
    (91, 'Northern'),
    (92, 'Grampian'),
    (93, 'Tayside'),
    (94, 'Fife'),
    (95, 'Lothian and Borders'),
    (96, 'Central'),
    (97, 'Strathclyde'),
    (98, 'Dumfries and Galloway'),
    (99, 'Police Scotland')
ON CONFLICT (name) DO NOTHING;



-- ---- SEVERITIES ---- --
-- -------------------- --
CREATE TABLE IF NOT EXISTS accidents.severities
(
    id                  INTEGER         NOT NULL        PRIMARY KEY,
    name                TEXT            NOT NULL        UNIQUE,
    is_enhanced         BOOL            NOT NULL        DEFAULT false
);

INSERT INTO accidents.severities (id, name, is_enhanced)
VALUES
    (1, 'Fatal', false),
    (2, 'Serious', false),
    (3, 'Slight', false),
    (5, 'Very Serious', true),
    (6, 'Moderately Serious', true),
    (7, 'Less Serious', true)
ON CONFLICT (name) DO NOTHING;



-- ---- CASUALTY CLASSES ---- --
-- -------------------------- --
CREATE TABLE IF NOT EXISTS accidents.casualty_classes
(
    id                  INTEGER         NOT NULL        PRIMARY KEY,
    name                TEXT            NOT NULL        UNIQUE
);

INSERT INTO accidents.casualty_classes (id, name)
VALUES
    (1, 'Driver/Rider'),
    (2, 'Passenger'),
    (3, 'Pedestrian')
ON CONFLICT (name) DO NOTHING;



-- ---- CASUALTY TYPES ---- --
-- ------------------------ --
CREATE TABLE IF NOT EXISTS accidents.casualty_types
(
    id                  INTEGER         NOT NULL        PRIMARY KEY,
    name                TEXT            NOT NULL        UNIQUE
);

INSERT INTO accidents.casualty_types (id, name)
VALUES
    (0, 'Pedestrian'),
    (1, 'Cyclist'),
    (2, 'Motorcycle 50cc and under rider or passenger'),
    (3, 'Motorcycle 125cc and under rider or passenger'),
    (4, 'Motorcycle over 125cc and up to 500cc rider or passenger'),
    (5, 'Motorcycle over 500cc rider or passenger'),
    (8, 'Taxi/Private hire car occupant'),
    (9, 'Car occupant'),
    (10, 'Minibus (8 - 16 passenger seats) occupant'),
    (11, 'Bus or coach occupant (17 or more pass seats)'),
    (16, 'Horse rider'),
    (17, 'Agricultural vehicle occupant'),
    (18, 'Tram occupant'),
    (19, 'Van / Goods vehicle (3.5 tonnes mgw or under) occupant'),
    (20, 'Goods vehicle (over 3.5t. and under 7.5t.) occupant'),
    (21, 'Goods vehicle (7.5 tonnes mgw and over) occupant'),
    (22, 'Mobility scooter rider'),
    (23, 'Electric motorcycle rider or passenger'),
    (90, 'Other vehicle occupant'),
    (97, 'Motorcycle - unknown cc rider or passenger'),
    (98, 'Goods vehicle (unknown weight) occupant'),
    (99, 'Unknown vehicle type (self rep only)'),
    (103, 'Motorcycle - Scooter (1979-1998)'),
    (104, 'Motorcycle (1979-1998)'),
    (105, 'Motorcycle - Combination (1979-1998)'),
    (106, 'Motorcycle over 125cc (1999-2004)'),
    (108, 'Taxi (excluding private hire cars) (1979-2004)'),
    (109, 'Car (including private hire cars) (1979-2004)'),
    (110, 'Minibus/Motor caravan (1979-1998)'),
    (113, 'Goods over 3.5 tonnes (1979-1998)')
ON CONFLICT (name) DO NOTHING;



-- ---- PEDESTRIAN LOCATIONS ---- --
-- ------------------------------ --
CREATE TABLE IF NOT EXISTS accidents.pedestrian_locations
(
    id                  INTEGER         NOT NULL        PRIMARY KEY,
    name                TEXT            NOT NULL        UNIQUE
);

INSERT INTO accidents.pedestrian_locations (id, name)
VALUES
    (0, 'Not a Pedestrian'),
    (1, 'Crossing on pedestrian crossing facility'),
    (2, 'Crossing in zig-zag approach lines'),
    (3, 'Crossing in zig-zag exit lines'),
    (4, 'Crossing elsewhere within 50m of pedestrian crossing'),
    (5, 'In carriageway, crossing elsewhere'),
    (6, 'On footway or verge'),
    (7, 'On refuge, central island or central reservation'),
    (8, 'In centre of carriageway - not on refuge, island or central reservation'),
    (9, 'In carriageway, not crossing'),
    (10, 'Unknown or other')
ON CONFLICT (name) DO NOTHING;



-- ---- PEDESTRIAN MOVEMENTS ---- --
-- ------------------------------ --
CREATE TABLE IF NOT EXISTS accidents.pedestrian_movements
(
    id                  INTEGER         NOT NULL        PRIMARY KEY,
    name                TEXT            NOT NULL        UNIQUE
);

INSERT INTO accidents.pedestrian_movements (id, name)
VALUES
    (0, 'Not a Pedestrian'),
    (1, 'Crossing from driver''s nearside'),
    (2, 'Crossing from nearside - masked by parked or stationary vehicle'),
    (3, 'Crossing from driver''s offside'),
    (4, 'Crossing from offside - masked by parked or stationary vehicle'),
    (5, 'In carriageway, stationary - not crossing (standing or playing)'),
    (6, 'In carriageway, stationary - not crossing (standing or playing) - masked by parked or stationary vehicle'),
    (7, 'Walking along in carriageway, facing traffic'),
    (8, 'Walking along in carriageway, back to traffic'),
    (9, 'Unknown or other')
ON CONFLICT (name) DO NOTHING;



-- ---- CAR PASSENGER TYPES ---- --
-- ----------------------------- --
CREATE TABLE IF NOT EXISTS accidents.car_passenger_types
(
    id                  INTEGER         NOT NULL        PRIMARY KEY,
    name                TEXT            NOT NULL        UNIQUE
);

INSERT INTO accidents.car_passenger_types (id, name)
VALUES
    (0, 'Not car passenger'),
    (1, 'Front seat passenger'),
    (2, 'Rear seat passenger'),
    (9, 'Unknown (self reported)')
ON CONFLICT (name) DO NOTHING;



-- ---- BUS PASSENGER TYPES ---- --
-- ----------------------------- --
CREATE TABLE IF NOT EXISTS accidents.bus_passenger_types
(
    id                  INTEGER         NOT NULL        PRIMARY KEY,
    name                TEXT            NOT NULL        UNIQUE
);

INSERT INTO accidents.bus_passenger_types (id, name)
VALUES
    (0, 'Not a bus or coach passenger'),
    (1, 'Boarding'),
    (2, 'Alighting'),
    (3, 'Standing passenger'),
    (4, 'Seated passenger'),
    (9, 'Unknown (self reported)')
ON CONFLICT (name) DO NOTHING;



-- ---- JOURNEY PURPOSES ---- --
-- -------------------------- --
CREATE TABLE IF NOT EXISTS accidents.journey_purposes
(
    id                  INTEGER         NOT NULL        PRIMARY KEY,
    name                TEXT            NOT NULL        UNIQUE
);

INSERT INTO accidents.journey_purposes (id, name)
VALUES
    (1, 'Journey as part of work'),
    (2, 'Commuting to/from work'),
    (3, 'Taking pupil to/from school'),
    (4, 'Pupil riding to/from school'),
    (5, 'Other'),
    (6, 'Not known or not requested'),
    (7, 'Education and educational escort'),
    (8, 'Emergency vehicle (blue light) on response'),
    (9, 'Personal business or leisure'),
    (15, 'Other/Not known')
ON CONFLICT (name) DO NOTHING;



-- ---- LIGHT CONDITIONS ---- --
-- -------------------------- --
CREATE TABLE IF NOT EXISTS accidents.light_conditions
(
    id                  INTEGER         NOT NULL        PRIMARY KEY,
    name                TEXT            NOT NULL        UNIQUE
);

INSERT INTO accidents.light_conditions (id, name)
VALUES
    (1, 'Daylight'),
    (4, 'Darkness - Lights Lit'),
    (5, 'Darkness - Lights Unlit'),
    (6, 'Darkness - No Lighting'),
    (7, 'Darkness - Lighting Unknown')
ON CONFLICT (name) DO NOTHING;



-- ---- WEATHER CONDITIONS ---- --
-- ---------------------------- --
CREATE TABLE IF NOT EXISTS accidents.weather_conditions
(
    id                  INTEGER         NOT NULL        PRIMARY KEY,
    name                TEXT            NOT NULL        UNIQUE
);

INSERT INTO accidents.weather_conditions (id, name)
VALUES
    (1, 'Fine - No High Winds'),
    (2, 'Raining - No High Winds'),
    (3, 'Snowing - No High Winds'),
    (4, 'Fine + High Winds'),
    (5, 'Raining + High Winds'),
    (6, 'Snowing + High Winds'),
    (7, 'Fog or Mist'),
    (8, 'Other')
ON CONFLICT (name) DO NOTHING;



-- ---- ROAD CONDITIONS ---- --
-- ------------------------- --
CREATE TABLE IF NOT EXISTS accidents.road_conditions
(
    id                  INTEGER         NOT NULL        PRIMARY KEY,
    name                TEXT            NOT NULL        UNIQUE
);

INSERT INTO accidents.road_conditions (id, name)
VALUES
    (1, 'Dry'),
    (2, 'Wet or Damp'),
    (3, 'Snow'),
    (4, 'Frost or Ice'),
    (5, 'Flood Over 3cm Deep'),
    (6, 'Oil/Diesel'),
    (7, 'Mud'),
    (9, 'Unknown')
ON CONFLICT (name) DO NOTHING;



-- ---- HAZARDS ---- --
-- ----------------- --
CREATE TABLE IF NOT EXISTS accidents.hazards
(
    id                  INTEGER         NOT NULL        PRIMARY KEY,
    name                TEXT            NOT NULL        UNIQUE
);

INSERT INTO accidents.hazards (id, name)
VALUES
    (0, 'None'),
    (1, 'Vehicle Load On Road'),
    (2, 'Other Object On Road'),
    (3, 'Previous Accident'),
    (4, 'Dog On Road'),
    (5, 'Other Animal On Road'),
    (6, 'Pedestrian In Carriageway - Not Injured'),
    (7, 'Any Animal In Carriageway (Except Ridden Horse)'),
    (11, 'Defective traffic signals'),
    (12, 'Permanent road signing or markings defective or obscured or inadequate'),
    (13, 'Roadworks'),
    (14, 'Oil or diesel'),
    (15, 'Mud'),
    (16, 'Dislodged vehicle load in carriageway'),
    (18, 'Involvement with previous collision'),
    (21, 'Poor or defective road surface'),
    (99, 'Unknown')
ON CONFLICT (name) DO NOTHING;



-- ---- SPECIAL CONDITIONS ---- --
-- ---------------------------- --
CREATE TABLE IF NOT EXISTS accidents.special_conditions
(
    id                  INTEGER         NOT NULL        PRIMARY KEY,
    name                TEXT            NOT NULL        UNIQUE
);

INSERT INTO accidents.special_conditions (id, name)
VALUES
    (0, 'None'),
    (1, 'Auto traffic signal - out'),
    (2, 'Auto signal part defective'),
    (3, 'Road sign or marking defective or obscured'),
    (4, 'Roadworks'),
    (5, 'Road surface defective'),
    (6, 'Oil or diesel'),
    (7, 'Mud'),
    (9, 'Unknown (self reported)')
ON CONFLICT (name) DO NOTHING;



-- ---- ROAD TYPES ---- --
-- -------------------- --
CREATE TABLE IF NOT EXISTS accidents.road_types
(
    id                  INTEGER         NOT NULL        PRIMARY KEY,
    name                TEXT            NOT NULL        UNIQUE
);

INSERT INTO accidents.road_types (id, name)
VALUES
    (1, 'Roundabout'),
    (2, 'One way street'),
    (3, 'Dual carriageway'),
    (6, 'Single carriageway'),
    (7, 'Slip road'),
    (9, 'Unknown'),
    (12, 'One way street/Slip road')
ON CONFLICT (name) DO NOTHING;



-- ---- VEHICLE TYPES ---- --
-- ----------------------- --
CREATE TABLE IF NOT EXISTS accidents.vehicle_types
(
    id                  INTEGER         NOT NULL        PRIMARY KEY,
    name                TEXT            NOT NULL        UNIQUE
);

INSERT INTO accidents.vehicle_types (id, name)
VALUES
    (1, 'Pedal cycle'),
    (2, 'Motorcycle 50cc and under'),
    (3, 'Motorcycle 125cc and under'),
    (4, 'Motorcycle over 125cc and up to 500cc'),
    (5, 'Motorcycle over 500cc'),
    (8, 'Taxi/Private hire car'),
    (9, 'Car'),
    (10, 'Minibus (8 - 16 passenger seats)'),
    (11, 'Bus or coach (17 or more pass seats)'),
    (16, 'Ridden horse'),
    (17, 'Agricultural vehicle'),
    (18, 'Tram'),
    (19, 'Van / Goods 3.5 tonnes mgw or under'),
    (20, 'Goods over 3.5t. and under 7.5t'),
    (21, 'Goods 7.5 tonnes mgw and over'),
    (22, 'Mobility scooter'),
    (23, 'Electric motorcycle'),
    (90, 'Other vehicle'),
    (97, 'Motorcycle - unknown cc'),
    (98, 'Goods vehicle - unknown weight'),
    (99, 'Unknown vehicle type (self rep only)'),
    (103, 'Motorcycle - Scooter (1979-1998)'),
    (104, 'Motorcycle (1979-1998)'),
    (105, 'Motorcycle - Combination (1979-1998)'),
    (106, 'Motorcycle over 125cc (1999-2004)'),
    (108, 'Taxi (excluding private hire cars) (1979-2004)'),
    (109, 'Car (including private hire cars) (1979-2004)'),
    (110, 'Minibus/Motor caravan (1979-1998)'),
    (113, 'Goods over 3.5 tonnes (1979-1998)')
ON CONFLICT (name) DO NOTHING;



-- ---- VEHICLE TOWING TYPES ---- --
-- ------------------------------ --
CREATE TABLE IF NOT EXISTS accidents.vehicle_towing_types
(
    id                  INTEGER         NOT NULL        PRIMARY KEY,
    name                TEXT            NOT NULL        UNIQUE
);

INSERT INTO accidents.vehicle_towing_types (id, name)
VALUES
    (0, 'None'),
    (1, 'Articulated vehicle'),
    (2, 'Double or multiple trailer'),
    (3, 'Caravan'),
    (4, 'Single trailer'),
    (5, 'Other tow')
ON CONFLICT (name) DO NOTHING;



-- ---- VEHICLE MANOEUVRES ---- --
-- ---------------------------- --
CREATE TABLE IF NOT EXISTS accidents.vehicle_manoeuvres
(
    id                  INTEGER         NOT NULL        PRIMARY KEY,
    name                TEXT            NOT NULL        UNIQUE
);

INSERT INTO accidents.vehicle_manoeuvres (id, name)
VALUES
    (1, 'Reversing'),
    (2, 'Parked'),
    (3, 'Waiting to go ahead'),
    (4, 'Slowing or stopping'),
    (5, 'Moving off'),
    (6, 'U-turn'),
    (7, 'Turning left'),
    (8, 'Waiting to turn left'),
    (9, 'Turning right'),
    (10, 'Waiting to turn right'),
    (11, 'Changing lane to left'),
    (12, 'Changing lane to right'),
    (13, 'Over taking moving vehicle on its offside'),
    (14, 'Overtaking stationary vehicle on its offside'),
    (15, 'Overtaking on nearside (passengers side nearest kerb)'),
    (16, 'Going ahead left-hand bend'),
    (17, 'Going ahead right-hand bend'),
    (18, 'Going ahead other'),
    (19, 'Going ahead'),
    (20, 'Parking')
ON CONFLICT (name) DO NOTHING;



-- ---- VEHICLE PROPULSION TYPES ---- --
-- ---------------------------------- --
CREATE TABLE IF NOT EXISTS accidents.vehicle_propulsion_types
(
    id                  INTEGER         NOT NULL        PRIMARY KEY,
    name                TEXT            NOT NULL        UNIQUE
);

INSERT INTO accidents.vehicle_propulsion_types (id, name)
VALUES
    (1, 'Petrol'),
    (2, 'Heavy oil'),
    (3, 'Electric'),
    (4, 'Steam'),
    (5, 'Gas'),
    (6, 'Petrol/Gas (LPG)'),
    (7, 'Gas/Bi-fuel'),
    (8, 'Hybrid electric'),
    (9, 'Gas Diesel'),
    (10, 'New fuel technology'),
    (11, 'Fuel cells'),
    (12, 'Electric diesel')
ON CONFLICT (name) DO NOTHING;



-- ---- VEHICLE HIT OBJECT "ON" ROAD TYPES ---- --
-- -------------------------------------------- --
CREATE TABLE IF NOT EXISTS accidents.vehicle_hit_object_on_road_types
(
    id                  INTEGER         NOT NULL        PRIMARY KEY,
    name                TEXT            NOT NULL        UNIQUE
);

INSERT INTO accidents.vehicle_hit_object_on_road_types (id, name)
VALUES
    (0, 'None'),
    (1, 'Previous accident'),
    (2, 'Road works'),
    (4, 'Parked vehicle'),
    (5, 'Bridge (roof)'),
    (6, 'Bridge (side)'),
    (7, 'Bollard or refuge'),
    (8, 'Open door of vehicle'),
    (9, 'Central island of roundabout'),
    (10, 'Kerb'),
    (11, 'Other object'),
    (12, 'Any animal (except ridden horse)'),
    (99, 'Unknown (self reported)')
ON CONFLICT (name) DO NOTHING;



-- ---- VEHICLE HIT OBJECT "OFF" ROAD TYPES ---- --
-- --------------------------------------------- --
CREATE TABLE IF NOT EXISTS accidents.vehicle_hit_object_off_road_types
(
    id                  INTEGER         NOT NULL        PRIMARY KEY,
    name                TEXT            NOT NULL        UNIQUE
);

INSERT INTO accidents.vehicle_hit_object_off_road_types (id, name)
VALUES
    (0, 'None'),
    (1, 'Road sign or traffic signal'),
    (2, 'Lamp post'),
    (3, 'Telegraph or electricity pole'),
    (4, 'Tree'),
    (5, 'Bus stop or bus shelter'),
    (6, 'Central crash barrier'),
    (7, 'Near/Offside crash barrier'),
    (8, 'Submerged in water'),
    (9, 'Entered ditch'),
    (10, 'Other permanent object'),
    (11, 'Wall or fence'),
    (99, 'Unknown (self reported)')
ON CONFLICT (name) DO NOTHING;



-- ---- VEHICLE POINT OF IMPACT TYPES ---- --
-- --------------------------------------- --
CREATE TABLE IF NOT EXISTS accidents.vehicle_point_of_impact_types
(
    id                  INTEGER         NOT NULL        PRIMARY KEY,
    name                TEXT            NOT NULL        UNIQUE
);

INSERT INTO accidents.vehicle_point_of_impact_types (id, name)
VALUES
    (0, 'Did not impact'),
    (1, 'Front'),
    (2, 'Back'),
    (3, 'Offside'),
    (4, 'Nearside'),
    (9, 'Unknown (self reported)')
ON CONFLICT (name) DO NOTHING;



-- ---- VEHICLE SKID OVERTURNING TYPES ---- --
-- ---------------------------------------- --
CREATE TABLE IF NOT EXISTS accidents.vehicle_skid_overturning_types
(
    id                  INTEGER         NOT NULL        PRIMARY KEY,
    name                TEXT            NOT NULL        UNIQUE
);

INSERT INTO accidents.vehicle_skid_overturning_types (id, name)
VALUES
    (0, 'None'),
    (1, 'Skidded'),
    (2, 'Skidded and overturned'),
    (3, 'Jackknifed'),
    (4, 'Jackknifed and overturned'),
    (5, 'Overturned'),
    (9, 'Unknown (self reported)')
ON CONFLICT (name) DO NOTHING;



-- ---- VEHICLE LEAVING ROAD TYPES ---- --
-- ------------------------------------ --
CREATE TABLE IF NOT EXISTS accidents.vehicle_leaving_road_types
(
    id                  INTEGER         NOT NULL        PRIMARY KEY,
    name                TEXT            NOT NULL        UNIQUE
);

INSERT INTO accidents.vehicle_leaving_road_types (id, name)
VALUES
    (0, 'Did not leave carriageway'),
    (1, 'Nearside'),
    (2, 'Nearside and rebounded'),
    (3, 'Straight ahead at junction'),
    (4, 'Offside on to central reservation'),
    (5, 'Offside on to central reservation + rebounded'),
    (6, 'Offside - crossed central reservation'),
    (7, 'Offside'),
    (8, 'Offside and rebounded'),
    (9, 'Unknown (self reported)')
ON CONFLICT (name) DO NOTHING;



-- ---- POLICE ATTENDANCE TYPES ---- --
-- --------------------------------- --
CREATE TABLE IF NOT EXISTS accidents.police_attendance_types
(
    id                  INTEGER         NOT NULL        PRIMARY KEY,
    name                TEXT            NOT NULL        UNIQUE
);

INSERT INTO accidents.police_attendance_types (id, name)
VALUES
    (1, 'Attended'),
    (2, 'Did not attend'),
    (3, 'Did not attend - accident self reported using form')
ON CONFLICT (name) DO NOTHING;



-- ---- COLLISIONS ---- --
-- -------------------- --
CREATE TABLE IF NOT EXISTS accidents.collisions
(
    id                          SERIAL          NOT NULL        PRIMARY KEY,
    collision_index             TEXT            NOT NULL        UNIQUE,
    ref_no                      TEXT            NOT NULL        UNIQUE,
    date                        TIMESTAMP       NOT NULL,
    location                    POINT           NOT NULL,
    police_force_id             INTEGER         NOT NULL,
    severity_id                 INTEGER         NOT NULL,
    road_name                   TEXT            NOT NULL,
    road_type_id                INTEGER         NOT NULL,
    speed_limit                 SMALLINT        NOT NULL,
    light_condition_id          INTEGER         NOT NULL,
    weather_condition_id        INTEGER         NOT NULL,
    road_condition_id           INTEGER         NOT NULL,
    special_condition_id        INTEGER         NOT NULL,
    hazard_id                   INTEGER         NOT NULL,
    is_rural                    BOOLEAN         NULL            DEFAULT NULL,
    police_attendance_type_id   INTEGER         NOT NULL,
    is_trunk_road               BOOLEAN         NULL            DEFAULT NULL,

    FOREIGN KEY (police_force_id) REFERENCES accidents.police_forces(id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION,

    FOREIGN KEY (severity_id) REFERENCES accidents.severities(id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION,

    FOREIGN KEY (road_type_id) REFERENCES accidents.road_types(id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION,

    FOREIGN KEY (light_condition_id) REFERENCES accidents.light_conditions(id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION,

    FOREIGN KEY (weather_condition_id) REFERENCES accidents.weather_conditions(id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION,

    FOREIGN KEY (road_condition_id) REFERENCES accidents.road_conditions(id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION,

    FOREIGN KEY (special_condition_id) REFERENCES accidents.special_conditions(id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION,

    FOREIGN KEY (hazard_id) REFERENCES accidents.hazards(id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION,

    FOREIGN KEY (police_attendance_type_id) REFERENCES accidents.police_attendance_types(id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION
);



-- ---- VEHICLES ---- --
-- ------------------ --
CREATE TABLE IF NOT EXISTS accidents.vehicles
(
    id                          SERIAL          NOT NULL        PRIMARY KEY,
    collision_id                INTEGER         NOT NULL,
    ref_no                      SMALLINT        NOT NULL,
    type_id                     INTEGER         NOT NULL,
    towing_type_id              INTEGER         NOT NULL,
    manoeuvre_id                INTEGER         NOT NULL,
    direction_from              VARCHAR(2)      NULL            CHECK (direction_from IN ('N', 'NE', 'E', 'SE', 'S', 'SW', 'W', 'NW', 'U')),
    direction_to                VARCHAR(2)      NULL            CHECK (direction_from IN ('N', 'NE', 'E', 'SE', 'S', 'SW', 'W', 'NW', 'U')),
    skid_overturning_type_id    INTEGER         NOT NULL,
    hit_object_on_road_type_id  INTEGER         NOT NULL,
    leaving_road_type_id        INTEGER         NOT NULL,
    hit_object_off_road_type_id INTEGER         NOT NULL,
    point_of_impact_type_id     INTEGER         NOT NULL,
    is_left_hand_drive          BOOLEAN         NULL            DEFAULT false,
    journey_purpose_id          INTEGER         NOT NULL,
    is_driver_male              BOOLEAN         NULL            DEFAULT NULL,
    driver_age                  SMALLINT        NULL            DEFAULT NULL,
    engine_capacity             SMALLINT        NULL            DEFAULT NULL,
    propulsion_type_id          INTEGER         NULL            DEFAULT NULL,
    vehicle_age                 SMALLINT        NULL            DEFAULT NULL,
    make_model                  TEXT            NULL            DEFAULT NULL,

    FOREIGN KEY (collision_id) REFERENCES accidents.collisions(id)
        ON DELETE CASCADE
        ON UPDATE NO ACTION,

    FOREIGN KEY (type_id) REFERENCES accidents.vehicle_types(id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION,

    FOREIGN KEY (towing_type_id) REFERENCES accidents.vehicle_towing_types(id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION,

    FOREIGN KEY (manoeuvre_id) REFERENCES accidents.vehicle_manoeuvres(id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION,

    FOREIGN KEY (skid_overturning_type_id) REFERENCES accidents.vehicle_skid_overturning_types(id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION,

    FOREIGN KEY (hit_object_on_road_type_id) REFERENCES accidents.vehicle_hit_object_on_road_types(id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION,

    FOREIGN KEY (leaving_road_type_id) REFERENCES accidents.vehicle_leaving_road_types(id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION,

    FOREIGN KEY (hit_object_off_road_type_id) REFERENCES accidents.vehicle_hit_object_off_road_types(id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION,

    FOREIGN KEY (point_of_impact_type_id) REFERENCES accidents.vehicle_point_of_impact_types(id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION,

    FOREIGN KEY (journey_purpose_id) REFERENCES accidents.journey_purposes(id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION,

    FOREIGN KEY (propulsion_type_id) REFERENCES accidents.vehicle_propulsion_types(id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION
);



-- ---- CASUALTIES ---- --
-- -------------------- --
CREATE TABLE IF NOT EXISTS accidents.casualties
(
    id                          SERIAL          NOT NULL        PRIMARY KEY,
    collision_id                INTEGER         NOT NULL,
    vehicle_id                  INTEGER         NOT NULL,
    ref_no                      SMALLINT        NOT NULL,
    class_id                    INTEGER         NOT NULL,
    is_male                     BOOLEAN         NULL            DEFAULT NULL,
    age                         SMALLINT        NULL            DEFAULT NULL,
    severity_id                 INTEGER         NOT NULL,
    pedestrian_location_id      INTEGER         NOT NULL,
    pedestrian_movement_id      INTEGER         NOT NULL,
    car_passenger_type_id       INTEGER         NOT NULL,
    bus_passenger_type_id       INTEGER         NOT NULL,
    type_id                     INTEGER         NOT NULL,

    FOREIGN KEY (collision_id) REFERENCES accidents.collisions(id)
        ON DELETE CASCADE
        ON UPDATE NO ACTION,

    FOREIGN KEY (vehicle_id) REFERENCES accidents.vehicles(id)
        ON DELETE CASCADE
        ON UPDATE NO ACTION,

    FOREIGN KEY (class_id) REFERENCES accidents.casualty_classes(id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION,

    FOREIGN KEY (severity_id) REFERENCES accidents.severities(id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION,

    FOREIGN KEY (pedestrian_location_id) REFERENCES accidents.pedestrian_locations(id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION,

    FOREIGN KEY (pedestrian_movement_id) REFERENCES accidents.pedestrian_movements(id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION,

    FOREIGN KEY (car_passenger_type_id) REFERENCES accidents.car_passenger_types(id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION,

    FOREIGN KEY (bus_passenger_type_id) REFERENCES accidents.bus_passenger_types(id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION,

    FOREIGN KEY (type_id) REFERENCES accidents.casualty_types(id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION
);