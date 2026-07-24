use std::time::Instant;
use std::{collections::HashMap};
use num_format::{Locale, ToFormattedString};
use sqlx::postgres::types::PgPoint;
use time::Date;
use time::macros::format_description;
use sqlx::{AssertSqlSafe, postgres::PgPoolOptions};
use sqlx::FromRow;
use sqlx::{Postgres, QueryBuilder};
use clap::Parser;
use serde::{Deserialize};

/// A CLI that processes UK traffic and accident data from the UK government
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Database connection string (e.g. "postgres://username:password@localhost")
    #[arg(short = 'c', long)]
    conn: String,

    /// Database name to use
    #[arg(short = 'd', long, default_value = "uk_traffic_accidents")]
    dbname: String,
}

struct Config {
    connection_string: String,
    database_name: String,
    sql_create_database: String,
    sql_create_traffic: String,
    sql_create_accident: String,
}

#[derive(FromRow, Debug)]
struct RoadCategoryDB {
    id: i32,
    name: String,
    description: String,
    is_major: bool,
}

#[derive(FromRow, Debug)]
struct RegionDB {
    id: i32,
    name: String,
    ons_code: String,
}

#[derive(FromRow, Debug)]
struct LocalAuthorityDB {
    id: i32,
    region_id: i32,
    name: String,
    ons_code: String,
}

#[derive(FromRow, Debug)]
struct CountPointDB {
    id: i32,
    local_authority_id: i32,
    road_category_id: i32,
    road_name: String,
    location: sqlx::postgres::types::PgPoint,
}

#[tokio::main]
async fn main() {
    let timer = Instant::now();

    let args = Args::parse();

    println!("------------------------------------------");
    println!("----- UK TRAFFIC & ACCIDENT IMPORTER -----");
    println!("------------------------------------------");
    println!("");
    println!("-- version: 0.1.0");
    println!("-- written by: Dominic Pettifer");
    println!("");

    let config = Config {
        connection_string: args.conn,
        database_name: args.dbname,
        sql_create_database: include_str!("../sql/1-create-database.sql").into(),
        sql_create_traffic: include_str!("../sql/2-create-traffic.sql").into(),
        sql_create_accident: include_str!("../sql/3-create-accidents.sql").into(),
    };

    print!("-- Building database: \"{}\"", &config.database_name);
    if let Err(e) = ensure_database_exists(&config).await {
        eprintln!("Error ensuring database exists: {:?}", e);
        std::process::exit(1);
    }
    print!(" ...done!\n");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect( &format!("{}/{}", config.connection_string, config.database_name))
        .await
        .expect("Error connecting to database");

    // load Road Categories
    print!("-- Loading Road Categories");
    let mut road_categories: HashMap<String, RoadCategoryDB> = HashMap::new();
    
    let road_cat_records: Vec<RoadCategoryDB> = sqlx::query_as("
        SELECT id, name, description, is_major
        FROM traffic.road_categories
        ORDER BY name ASC
    ")
    .fetch_all(&pool)
    .await
    .expect("Error loading Road Categories");

    for road_cat in road_cat_records {
        road_categories.insert(road_cat.name.clone(), road_cat);
    }
    print!(" ...done ({} records)\n", road_categories.len());

    // load Regions
    print!("-- Loading Regions");
    let mut regions: HashMap<i32, RegionDB> = HashMap::new();

    let region_records: Vec<RegionDB> = sqlx::query_as("
        SELECT id, name, ons_code
        FROM traffic.regions
        ORDER BY id ASC;
    ")
    .fetch_all(&pool)
    .await
    .expect("Error loading regions");

    for region in region_records {
        regions.insert(region.id, region);
    }

    print!(" ...done (");
    let total_regions = regions.len();
    if total_regions > 0 {
        print!("{} records)\n", total_regions);
    }
    else {
        print!("empty)\n");
    }

    // load Local Authorities
    print!("-- Loading Local Authorities");
    let mut local_authorities: HashMap<i32, LocalAuthorityDB> = HashMap::new();

    let local_authority_records: Vec<LocalAuthorityDB> = sqlx::query_as("
        SELECT id, region_id, name, ons_code
        FROM traffic.local_authorities
        ORDER BY id ASC;
    ")
    .fetch_all(&pool)
    .await
    .expect("Error loading authorities");

    for local_authority in local_authority_records {
        local_authorities.insert(local_authority.id, local_authority);
    }

    print!(" ...done (");
    let total_local_authorities = local_authorities.len();
    if total_local_authorities > 0 {
        print!("{} records)\n", total_local_authorities);
    }
    else {
        print!("empty)\n");
    }

    // load Count Points
    print!("-- Loading Count Points");
    let mut count_points: HashMap<i32, CountPointDB> = HashMap::new();

    let count_point_records: Vec<CountPointDB> = sqlx::query_as("
        SELECT id, local_authority_id, road_category_id, road_name, location
        FROM traffic.count_points
        ORDER BY id ASC;
    ")
    .fetch_all(&pool)
    .await
    .expect("Error loading Count Points");

    for count_point in count_point_records {
        count_points.insert(count_point.id, count_point);
    }

    print!(" ...done (");
    let total_count_points = count_points.len();
    if total_count_points > 0 {
        print!("{} records)\n", total_count_points);
    }
    else {
        print!("empty)\n");
    }

    println!("");

    // load CSV count points
    print!("-- Reading \"Count Points.csv\" file");
    let mut total_road_catregories_inserted = 0;
    let mut total_regions_inserted = 0;
    let mut total_local_authorities_inserted = 0;
    let mut total_count_points_inserted = 0;

    let count_points_reader_result = csv::Reader::from_path("data/traffic/Count Points.csv");
    if let Err(e) = count_points_reader_result {
        match e.kind() {
            csv::ErrorKind::Io(_) => eprintln!("\n   |-- Count Points.csv not found"),
            _ => eprintln!("\n   |-- Error opening file: {:?}", e),
        }

        std::process::exit(1);
    }

    for result in count_points_reader_result.unwrap().deserialize() {
        let count_point_csv: CountPointCSV = result.unwrap();

        let road_category = match road_categories.get(&count_point_csv.road_category) {
            Some(rc) => rc,
            None => {
                println!("   |-- Unknown Road Category detected: {}", count_point_csv.road_category);

                let road_category: RoadCategoryDB = sqlx::query_as("
                    INSERT INTO traffic.road_categories(name, description)
                    VALUES($1, $2)
                    RETURNING *;
                ")
                .bind(&count_point_csv.road_category)
                .bind("Unknown road category")
                .fetch_one(&pool)
                .await
                .expect("Error inserting road category");

                road_categories.insert(road_category.name.clone(), road_category);
                total_road_catregories_inserted += 1;

                road_categories.get(&count_point_csv.road_category).unwrap()
            }
        };

        let region = match regions.get(&count_point_csv.region_id) {
            Some(r) => r,
            None => {
                let region: RegionDB = sqlx::query_as("
                    INSERT INTO traffic.regions(id, name, ons_code)
                    VALUES($1, $2, $3)
                    RETURNING *;
                ")
                .bind(&count_point_csv.region_id)
                .bind(&count_point_csv.region_name)
                .bind(&count_point_csv.region_ons_code)
                .fetch_one(&pool)
                .await
                .expect("Error inserting region");

                regions.insert(count_point_csv.region_id, region);
                total_regions_inserted += 1;

                regions.get(&count_point_csv.region_id).unwrap()
            }
        };

        if region.name != count_point_csv.region_name || region.ons_code != count_point_csv.region_ons_code {
            panic!("Error region record mismatch");
        }

        let local_authority = match local_authorities.get(&count_point_csv.local_authority_id) {
            Some(la) => la,
            None => {
                let local_authority: LocalAuthorityDB = sqlx::query_as("
                    INSERT INTO traffic.local_authorities(id, region_id, name, ons_code)
                    VALUES($1, $2, $3, $4)
                    RETURNING *;
                ")
                .bind(&count_point_csv.local_authority_id)
                .bind(&region.id)
                .bind(&count_point_csv.local_authority_name)
                .bind(&count_point_csv.local_authority_code)
                .fetch_one(&pool)
                .await
                .expect("Error inserting local authority");

                local_authorities.insert(local_authority.id, local_authority);
                total_local_authorities_inserted += 1;

                local_authorities.get(&count_point_csv.local_authority_id).unwrap()
            }
        };

        if local_authority.name != count_point_csv.local_authority_name ||
            local_authority.ons_code != count_point_csv.local_authority_code {
            panic!("Error: Local Authority record mismatch");
        }

        let location = PgPoint {
            x: count_point_csv.latitude,
            y: count_point_csv.longitude
        };

        if !count_points.contains_key(&count_point_csv.count_point_id) {
            let count_point: CountPointDB = sqlx::query_as("
                INSERT INTO traffic.count_points (id, local_authority_id, road_category_id, road_name, location)
                VALUES($1, $2, $3, $4, $5)
                RETURNING *;
            ")
            .bind(&count_point_csv.count_point_id)
            .bind(&local_authority.id)
            .bind(&road_category.id)
            .bind(&count_point_csv.road_name)
            .bind(&location)
            .fetch_one(&pool)
            .await
            .expect("Error inserting Count Point");

            count_points.insert(count_point.id, count_point);
            total_count_points_inserted += 1;
        }
    }

    print!(" ...done!\n");
    println!("   |-- {} new Road Categories inserted", total_road_catregories_inserted);
    println!("   |-- {} new Regions inserted", total_regions_inserted);
    println!("   |-- {} new Local Authorities inserted", total_local_authorities_inserted);
    println!("   |-- {} new Count Points inserted", total_count_points_inserted);

    println!("");

    // delete all counts
    print!("-- Clearing Counts table");

    sqlx::query("TRUNCATE TABLE traffic.counts;")
        .execute(&pool)
        .await
        .expect("Error truncating traffic counts.");

    print!(" ...done!\n");
    println!("");

    // read Raw Counts.csv file
    println!("-- Reading \"Raw counts.csv\" file (this will take a while)...");

    let reader_result = csv::Reader::from_path("data/traffic/Raw counts.csv");
    if let Err(e) = reader_result {
        match e.kind() {
            csv::ErrorKind::Io(_) => eprintln!("Raw counts.csv not found"),
            _ => eprintln!("Error opening file: {:?}", e),
        }

        std::process::exit(1);
    }

    const BATCH_SIZE: usize = 2123;
    let mut batch: Vec<CountInsert> = Vec::with_capacity(BATCH_SIZE);

    let mut reader = reader_result.unwrap();
    let headers = reader.headers().unwrap().clone();
    let mut records_processed = 0;

    let header_map: HashMap<&str, usize> = headers
        .iter()
        .enumerate()
        .map(|(i, h)| (h, i))
        .collect();

    let date_format = format_description!("[year]-[month]-[day]");

    for result in reader.records() {
        let record = result.unwrap();

        let raw_count_result: Result<RawCountCSV, csv::Error> = record.deserialize(Some(&headers));

        if let Err(e) = raw_count_result {
            // ignore rows with invalid values as there's only about a dozen for millions of rows
            let count_has_na_value =
                record.get(header_map["pedal_cycles"]) == Some("NA") ||
                record.get(header_map["two_wheeled_motor_vehicles"]) == Some("NA") ||
                record.get(header_map["cars_and_taxis"]) == Some("NA") ||
                record.get(header_map["buses_and_coaches"]) == Some("NA") ||
                record.get(header_map["LGVs"]) == Some("NA") ||
                record.get(header_map["all_HGVs"]) == Some("NA");

            let count_has_negative_value =
                record.get(header_map["pedal_cycles"]) == Some("-1") ||
                record.get(header_map["two_wheeled_motor_vehicles"]) == Some("-1") ||
                record.get(header_map["cars_and_taxis"]) == Some("-1") ||
                record.get(header_map["buses_and_coaches"]) == Some("-1") ||
                record.get(header_map["LGVs"]) == Some("-1") ||
                record.get(header_map["all_HGVs"]) == Some("-1");

            if count_has_na_value || count_has_negative_value {
                continue;
            }

            let info = e.position().unwrap();
            eprintln!("Error parsing row {}: {:?}. Error: {:?}", info.line(), record, e);
            std::process::exit(1);
        }

        let raw_count = raw_count_result.unwrap();

        let count_point = count_points.get(&raw_count.count_point_id)
            .expect(&format!("Count Point (ID: {}) doesn't exist in database", raw_count.count_point_id));

        let date = Date::parse(&raw_count.count_date, &date_format)
            .expect("Error pasing date");

        let direction = raw_count.direction_of_travel.trim().to_uppercase();

        let row = CountInsert {
            count_point_id: count_point.id,
            date: date,
            hour: raw_count.hour as i16,
            direction: direction,
            bicycles: raw_count.bicycles as i32,
            motorcycles: raw_count.motorcycles as i32,
            cars: raw_count.cars as i32,
            buses: raw_count.buses as i32,
            lgvs: raw_count.lgvs as i32,
            hgvs: raw_count.hgvs as i32,
        };

        batch.push(row);

        if batch.len() == BATCH_SIZE {
            insert_count_batch(&pool, &batch).await.unwrap();
            records_processed += batch.len() as u64;
            batch.clear();

            print!("\r   |-- {} records processed", records_processed.to_formatted_string(&Locale::en));
        }
    }

    if !batch.is_empty() {
        insert_count_batch(&pool, &batch).await.unwrap();
        records_processed += batch.len() as u64;

        print!("\r   |-- {} records processed", records_processed.to_formatted_string(&Locale::en));
    }

    println!("");
    println!("Finished! Took {:?}", timer.elapsed());
}



#[derive(Debug, Deserialize)]
struct RawCountCSV {
    count_point_id: i32,
    direction_of_travel: String,
    count_date: String,
    hour: u32,

    #[serde(rename = "pedal_cycles")]
    bicycles: u32,

    #[serde(rename = "two_wheeled_motor_vehicles")]
    motorcycles: u32,

    #[serde(rename = "cars_and_taxis")]
    cars: u32,

    #[serde(rename = "buses_and_coaches")]
    buses: u32,

    #[serde(rename = "LGVs")]
    lgvs: u32,

    #[serde(rename = "all_HGVs")]
    hgvs: u32,
}

#[derive(Debug, Deserialize)]
struct CountPointCSV {
    count_point_id: i32,
    region_id: i32,
    region_name: String,
    region_ons_code: String,
    local_authority_id: i32,
    local_authority_name: String,
    local_authority_code: String,
    road_name: String,
    road_category: String,
    latitude: f64,
    longitude: f64,
}

struct CountInsert {
    count_point_id: i32,
    date: time::Date,
    hour: i16,
    direction: String,
    bicycles: i32,
    motorcycles: i32,
    cars: i32,
    buses: i32,
    lgvs: i32,
    hgvs: i32,
}

async fn insert_count_batch(pool: &sqlx::PgPool, rows: &[CountInsert]) -> Result<(), sqlx::Error> {
    let mut query = QueryBuilder::<Postgres>::new("
        INSERT INTO traffic.counts
        (
            count_point_id,
            date,
            hour,
            direction,
            bicycles,
            motorcycles,
            cars,
            buses,
            lgvs,
            hgvs
        )
    ");

    query.push_values(rows, |mut values, row| {
        values
            .push_bind(row.count_point_id)
            .push_bind(row.date)
            .push_bind(row.hour)
            .push_bind(&row.direction)
            .push_bind(row.bicycles)
            .push_bind(row.motorcycles)
            .push_bind(row.cars)
            .push_bind(row.buses)
            .push_bind(row.lgvs)
            .push_bind(row.hgvs);
    });

    query.build().execute(pool).await?;

    return Ok(());
}

async fn ensure_database_exists(config: &Config) -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&format!("{}/postgres", config.connection_string))
        .await?;

    let exists: bool = sqlx::query_scalar(
        "
        SELECT EXISTS (
            SELECT 1
            FROM pg_database
            WHERE datname = $1
        )"
    )
    .bind(&config.database_name)
    .fetch_one(&pool)
    .await?;

    if !exists {
        // create database
        sqlx::query(AssertSqlSafe(config.sql_create_database.clone()))
            .execute(&pool)
            .await?;

        // connect to new database and create tables
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect( &format!("{}/{}", config.connection_string, config.database_name))
            .await?;

        sqlx::raw_sql(AssertSqlSafe(config.sql_create_traffic.clone()))
            .execute(&pool)
            .await?;

        sqlx::raw_sql(AssertSqlSafe(config.sql_create_accident.clone()))
            .execute(&pool)
            .await?;
    }

    return Ok(());
}