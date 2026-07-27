pub mod database;
pub mod csv_utils;
pub mod models;

use std::io::Write;
use std::time::{Instant};
use std::{collections::HashMap};

use num_format::{Locale, ToFormattedString};
use clap::Parser;
use time::Date;
use time::macros::format_description;

use sqlx::postgres::types::PgPoint;
use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::database::{CountDB, CountPointDB, LocalAuthorityDB, RegionDB, RoadCategoryDB};
use crate::csv_utils::{CountPointCSV, RawCountCSV};
use crate::models::{Config};

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

#[tokio::main]
async fn main() {
    let timer = Instant::now();

    let args = Args::parse();

    println!("------------------------------------------");
    println!("----- UK TRAFFIC & ACCIDENT IMPORTER -----");
    println!("------------------------------------------");
    println!("");
    println!("version: 0.2.0");
    println!("written by: Dominic Pettifer");
    println!("");

    let config = Config {
        connection_string: args.conn,
        database_name: args.dbname,
        sql_create_database: include_str!("../sql/1-create-database.sql").into(),
        sql_create_traffic: include_str!("../sql/2-create-traffic.sql").into(),
        sql_create_accident: include_str!("../sql/3-create-accidents.sql").into(),
    };

    // create database
    print!("-- Building database: \"{}\"", &config.database_name);
    std::io::stdout().flush().unwrap();

    if let Err(e) = crate::database::ensure_database_exists(&config).await {
        eprintln!("Error ensuring database exists: {:?}", e);
        std::process::exit(1);
    }
    print!(" ...done!");
    println!("");

    // connect to database
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect( &format!("{}/{}", config.connection_string, config.database_name))
        .await
        .expect("Error connecting to database");

    // load Road Categories
    print!("-- Loading Road Categories");
    std::io::stdout().flush().unwrap();

    let mut road_categories = RoadCategoryDB::load_from_db_hashmap(&pool).await
        .expect("Error loading Road Categories");
    print!(" ...done ({} records)\n", road_categories.len());

    // load Regions
    print!("-- Loading Regions");
    std::io::stdout().flush().unwrap();

    let mut regions = RegionDB::load_from_db_hashmap(&pool).await
        .expect("Error loading Regions");
    print!(" ...done");

    let total_regions = regions.len();
    if total_regions > 0 {
        print!(" ({} records)\n", total_regions);
    }
    else {
        print!(" (empty)\n");
    }

    // load Local Authorities
    print!("-- Loading Local Authorities");
    std::io::stdout().flush().unwrap();

    let mut local_authorities = LocalAuthorityDB::load_from_db_hashmap(&pool).await
        .expect("Error loading Local Authorities");
    print!(" ...done");

    let total_local_authorities = local_authorities.len();
    if total_local_authorities > 0 {
        print!(" ({} records)\n", total_local_authorities);
    }
    else {
        print!(" (empty)\n");
    }

    // load Count Points
    print!("-- Loading Count Points");
    std::io::stdout().flush().unwrap();

    let mut count_points = CountPointDB::load_from_db_hashmap(&pool).await
        .expect("Error loading Count Points");
    print!(" ...done");

    let total_count_points = count_points.len();
    if total_count_points > 0 {
        print!(" ({} records)\n", total_count_points);
    }
    else {
        print!(" (empty)\n");
    }

    println!("");

    // load CSV count points
    println!("-- Reading \"Count Points.csv\" file");

    const COUNT_POINT_BATCH_SIZE: usize = 1234;
    let mut count_point_batch: Vec<CountPointDB> = Vec::with_capacity(COUNT_POINT_BATCH_SIZE);

    let mut total_road_catregories_inserted = 0;
    let mut total_regions_inserted = 0;
    let mut total_local_authorities_inserted = 0;
    let mut total_count_points_inserted = 0;

    const COUNT_POINTS_PATH: &str = "data/traffic/Count Points.csv";
    let mut count_points_reader = match csv::Reader::from_path(COUNT_POINTS_PATH) {
        Ok(reader) => reader,
        Err(e) => {
            match e.kind() {
                csv::ErrorKind::Io(_) => eprintln!("\n   |-- \"{}\" not found", COUNT_POINTS_PATH),
                _ => eprintln!("\n   |-- Error opening \"{}\": {:?}", COUNT_POINTS_PATH, e),
            }

            std::process::exit(1);
        }
    };

    for result in count_points_reader.deserialize() {
        let count_point_csv: CountPointCSV = result.unwrap();

        let road_category = match road_categories.get(&count_point_csv.road_category) {
            Some(rc) => rc,
            None => {
                println!("   |-- Unknown Road Category detected: {}", count_point_csv.road_category);

                let road_category = RoadCategoryDB::insert(&pool,
                    &count_point_csv.road_category,
                    "Unknown road category"
                )
                .await
                .expect("Error inserting Road Category");

                road_categories.insert(road_category.name.clone(), road_category);
                total_road_catregories_inserted += 1;

                road_categories.get(&count_point_csv.road_category).unwrap()
            }
        };

        let region = match regions.get(&count_point_csv.region_id) {
            Some(r) => r,
            None => {
                let region = RegionDB::insert(&pool,
                    count_point_csv.region_id,
                    &count_point_csv.region_name,
                    &count_point_csv.region_ons_code
                )
                .await
                .expect("Error inserting Region");

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
                let local_authority = LocalAuthorityDB::insert(&pool,
                    count_point_csv.local_authority_id,
                    region.id,
                    &count_point_csv.local_authority_name,
                    &count_point_csv.local_authority_code
                )
                .await
                .expect("Error inserting Local Authority");

                local_authorities.insert(local_authority.id, local_authority);
                total_local_authorities_inserted += 1;

                local_authorities.get(&count_point_csv.local_authority_id).unwrap()
            }
        };

        if local_authority.name != count_point_csv.local_authority_name ||
            local_authority.ons_code != count_point_csv.local_authority_code {
            panic!("Error: Local Authority record mismatch");
        }

        if !count_points.contains_key(&count_point_csv.count_point_id) {
            let count_point = CountPointDB {
                id: count_point_csv.count_point_id,
                local_authority_id: local_authority.id,
                road_category_id: road_category.id,
                road_name: count_point_csv.road_name,
                location: PgPoint {
                    x: count_point_csv.latitude,
                    y: count_point_csv.longitude
                },
            };

            count_point_batch.push(count_point);

            if count_point_batch.len() == COUNT_POINT_BATCH_SIZE {
                CountPointDB::insert_batch(&pool, &count_point_batch).await.unwrap();
                total_count_points_inserted += count_point_batch.len() as u64;
                count_point_batch.clear();

                print!("\r   |-- {} new Count Points inserted", total_count_points_inserted.to_formatted_string(&Locale::en));
                std::io::stdout().flush().unwrap();
            }
        }
    }

    if !count_point_batch.is_empty() {
        CountPointDB::insert_batch(&pool, &count_point_batch).await.unwrap();
        total_count_points_inserted += count_point_batch.len() as u64;

        print!("\r   |-- {} new Count Points inserted", total_count_points_inserted.to_formatted_string(&Locale::en));
    }

    if total_count_points_inserted == 0 {
        println!("   |-- 0 new Count Points inserted");
    }
    else {
        println!("");
    }

    println!("   |-- {} new Road Categories inserted", total_road_catregories_inserted);
    println!("   |-- {} new Regions inserted", total_regions_inserted);
    println!("   |-- {} new Local Authorities inserted", total_local_authorities_inserted);
    println!("");

    if total_count_points_inserted > 0 {
        count_points = CountPointDB::load_from_db_hashmap(&pool).await
            .expect("Error loading Count Points");
    }

    // delete all counts
    print!("-- Clearing Counts table");
    std::io::stdout().flush().unwrap();

    CountDB::truncate_all(&pool).await
        .expect("Error truncating traffic counts.");

    print!(" ...done!\n");
    println!("");

    // read Raw Counts.csv file
    println!("-- Reading \"Raw counts.csv\" file (this will take a while)...");
    std::io::stdout().flush().unwrap();

    let reader_result = csv::Reader::from_path("data/traffic/Raw counts.csv");
    if let Err(e) = reader_result {
        match e.kind() {
            csv::ErrorKind::Io(_) => eprintln!("Raw counts.csv not found"),
            _ => eprintln!("Error opening file: {:?}", e),
        }

        std::process::exit(1);
    }

    const BATCH_SIZE: usize = 2123;
    let mut batch: Vec<CountDB> = Vec::with_capacity(BATCH_SIZE);

    let mut reader = reader_result.unwrap();
    let headers = reader.headers().unwrap().clone();
    let mut records_processed = 0;

    let header_map: HashMap<&str, usize> = headers
        .iter()
        .enumerate()
        .map(|(i, h)| (h, i))
        .collect();

    let date_format = format_description!("[year]-[month]-[day]");
    let allowed_directions = ["N", "E", "S", "W", "C", "J"];

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

            handle_count_csv_parse_error(Some(e), "Deserialization error", &pool).await;
            std::process::exit(1);
        }

        let raw_count = raw_count_result.unwrap();

        let count_point = match count_points.get(&raw_count.count_point_id) {
            Some(cp) => cp,
            None => {
                let message = format!("Count Point (ID: {}) doesn't exist in Count Points CSV", raw_count.count_point_id);
                handle_count_csv_parse_error(None, &message, &pool).await;
                std::process::exit(1);
            }
        };

        let date = match Date::parse(&raw_count.count_date, &date_format) {
            Ok(d) => d,
            Err(e) => {
                let message = format!("Error parsing date: {:?}. Raw date value: {}", e, &raw_count.count_date);
                handle_count_csv_parse_error(None, &message, &pool).await;
                std::process::exit(1);
            }
        };

        let direction = raw_count.direction_of_travel.trim().to_uppercase();
        if !allowed_directions.contains(&direction.as_str()) {
            let message = format!("Invalid direction: {}. Allowed directions are: {:?}", direction, allowed_directions);
            handle_count_csv_parse_error(None, &message, &pool).await;
            std::process::exit(1);
        }

        let row = CountDB {
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
            CountDB::insert_batch(&pool, &batch).await.unwrap();
            records_processed += batch.len() as u64;
            batch.clear();

            print!("\r   |-- {} records processed", records_processed.to_formatted_string(&Locale::en));
            std::io::stdout().flush().unwrap();
        }
    }

    if !batch.is_empty() {
        CountDB::insert_batch(&pool, &batch).await.unwrap();
        records_processed += batch.len() as u64;

        print!("\r   |-- {} records processed", records_processed.to_formatted_string(&Locale::en));
    }

    println!("");
    println!("");
    println!("Finished! Took {:?}", timer.elapsed());
}

async fn handle_count_csv_parse_error(e: Option<csv::Error>, message: &str, pool: &PgPool) {
    eprintln!("Error parsing row: {:?}.", message);

    if let Some(e) = e {
        let info = e.position().unwrap();
        eprintln!("CSV error (row: {}): {:?}", info.line(), e);
    }

    eprintln!("Counts table wil be truncated");

    if let Err(e) = CountDB::truncate_all(pool).await {
        eprintln!("Error truncating traffic counts: {:?}", e);
    }
}