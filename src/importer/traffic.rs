use crate::database::{CountDB, CountPointDB, LocalAuthorityDB, RegionDB, RoadCategoryDB};
use crate::csv_utils::{CountPointCSV, RawCountCSV};

use sqlx::PgPool;
use sqlx::postgres::types::PgPoint;

use std::io::Write;
use std::{collections::HashMap};

use num_format::{Locale, ToFormattedString};
use time::Date;
use time::macros::format_description;

struct TrafficData {
    road_categories: HashMap<String, RoadCategoryDB>,
    regions: HashMap<i32, RegionDB>,
    local_authorities: HashMap<i32, LocalAuthorityDB>,
    count_points: HashMap<i32, CountPointDB>,
}

pub struct TrafficImporter<'a> {
    pool: &'a PgPool,
}

impl<'a> TrafficImporter<'a> {
    pub fn new(pool: &'a PgPool) -> TrafficImporter<'a> {
        TrafficImporter {
            pool: pool,
        }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut data = self.load_data().await?;
        self.import_count_points(&mut data).await?;
        self.import_raw_counts(&mut data).await?;

        Ok(())
    }

    async fn load_data(&self) -> Result<TrafficData, Box<dyn std::error::Error>> {
        // load Road Categories
        print!("-- Loading Road Categories");
        std::io::stdout().flush().unwrap();

        let road_categories = match RoadCategoryDB::load_from_db_hashmap(self.pool).await {
            Ok(rc) => rc,
            Err(e) => return Err(format!("Error loading Road Categories: {e}").into()),
        };

        print!(" ...done ({} records)\n", road_categories.len());

        // load Regions
        print!("-- Loading Regions");
        std::io::stdout().flush().unwrap();

        let regions = match RegionDB::load_from_db_hashmap(self.pool).await {
            Ok(r) => r,
            Err(e) => return Err(format!("Error loading Regions: {e}").into()),
        };
   
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

        let local_authorities = match LocalAuthorityDB::load_from_db_hashmap(self.pool).await {
            Ok(la) => la,
            Err(e) => return Err(format!("Error loading Local Authorities: {e}").into()),
        };

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

        let count_points = match CountPointDB::load_from_db_hashmap(self.pool).await {
            Ok(cp) => cp,
            Err(e) => return Err(format!("Error loading Count Points: {e}").into()),
        };

        print!(" ...done");

        let total_count_points = count_points.len();
        if total_count_points > 0 {
            print!(" ({} records)\n", total_count_points);
        }
        else {
            print!(" (empty)\n");
        }

        println!("");

        Ok(TrafficData {
            road_categories: road_categories,
            regions: regions,
            local_authorities: local_authorities,
            count_points: count_points,
        })
    }

    async fn import_count_points(&self, data: &mut TrafficData) -> Result<(), Box<dyn std::error::Error>> {
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
                    csv::ErrorKind::Io(_) => return Err(format!("\n   |-- \"{}\" not found", COUNT_POINTS_PATH).into()),
                    _ => return Err(format!("\n   |-- Error opening \"{}\": {:?}", COUNT_POINTS_PATH, e).into()),
                }
            }
        };

        for result in count_points_reader.deserialize() {
            let count_point_csv: CountPointCSV = match result {
                Ok(csv) => csv,
                Err(e) => return Err(format!("Error deserializing Count Point: {e}").into()),
            };

            let road_category = match data.road_categories.get(&count_point_csv.road_category) {
                Some(rc) => rc,
                None => {
                    println!("   |-- Unknown Road Category detected: {}", count_point_csv.road_category);

                    let road_category = RoadCategoryDB::insert(self.pool,
                        &count_point_csv.road_category,
                        "Unknown road category"
                    )
                    .await
                    .map_err(|error| -> Box<dyn std::error::Error> {
                        format!("Error inserting Road Category: {error}").into()
                    })?;

                    data.road_categories.insert(road_category.name.clone(), road_category);
                    total_road_catregories_inserted += 1;

                    data.road_categories.get(&count_point_csv.road_category).unwrap()
                }
            };

            let region = match data.regions.get(&count_point_csv.region_id) {
                Some(r) => r,
                None => {
                    let region = RegionDB::insert(self.pool,
                        count_point_csv.region_id,
                        &count_point_csv.region_name,
                        &count_point_csv.region_ons_code
                    )
                    .await
                    .map_err(|error| -> Box<dyn std::error::Error> {
                        format!("Error inserting Region: {error}").into()
                    })?;

                    data.regions.insert(count_point_csv.region_id, region);
                    total_regions_inserted += 1;

                    data.regions.get(&count_point_csv.region_id).unwrap()
                }
            };

            if region.name != count_point_csv.region_name || region.ons_code != count_point_csv.region_ons_code {
                panic!("Error region record mismatch");
            }

            let local_authority = match data.local_authorities.get(&count_point_csv.local_authority_id) {
                Some(la) => la,
                None => {
                    let local_authority = LocalAuthorityDB::insert(self.pool,
                        count_point_csv.local_authority_id,
                        region.id,
                        &count_point_csv.local_authority_name,
                        &count_point_csv.local_authority_code
                    )
                    .await
                    .map_err(|error| -> Box<dyn std::error::Error> {
                        format!("Error inserting Local Authority: {error}").into()
                    })?;

                    data.local_authorities.insert(local_authority.id, local_authority);
                    total_local_authorities_inserted += 1;

                    data.local_authorities.get(&count_point_csv.local_authority_id).unwrap()
                }
            };

            if local_authority.name != count_point_csv.local_authority_name ||
                local_authority.ons_code != count_point_csv.local_authority_code {
                return Err("Error: Local Authority record mismatch".into());
            }

            if !data.count_points.contains_key(&count_point_csv.count_point_id) {
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
                    if let Err(e) = CountPointDB::insert_batch(self.pool, &count_point_batch).await {
                        return Err(format!("Error inserting batch of count points: {e}").into());
                    }

                    total_count_points_inserted += count_point_batch.len() as u64;
                    count_point_batch.clear();

                    print!("\r   |-- {} new Count Points inserted", total_count_points_inserted.to_formatted_string(&Locale::en));
                    std::io::stdout().flush().unwrap();
                }
            }
        }

        if !count_point_batch.is_empty() {
            if let Err(e) = CountPointDB::insert_batch(self.pool, &count_point_batch).await {
                return Err(format!("Error inserting batch of count points: {e}").into());
            }

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
            data.count_points = CountPointDB::load_from_db_hashmap(self.pool).await
                .map_err(|error| -> Box<dyn std::error::Error> {
                    format!("Error loading Count Points: {error}").into()
                })?;
        }

        Ok(())
    }

    async fn import_raw_counts(&self, data: &mut TrafficData) -> Result<(), Box<dyn std::error::Error>> {
        // delete all counts
        print!("-- Clearing Counts table");
        std::io::stdout().flush().unwrap();

        if let Err(e) = CountDB::truncate_all(self.pool).await {
            return Err(format!("Error truncating traffic counts: {e}").into());
        }

        print!(" ...done!\n");
        println!("");

        // read Raw Counts.csv file
        println!("-- Reading \"Raw counts.csv\" file (this will take a while)...");
        std::io::stdout().flush().unwrap();

        const BATCH_SIZE: usize = 2123;
        let mut batch: Vec<CountDB> = Vec::with_capacity(BATCH_SIZE);

        let mut reader = match csv::Reader::from_path("data/traffic/Raw counts.csv") {
            Ok(rr) => rr,
            Err(e) => {
                match e.kind() {
                    csv::ErrorKind::Io(_) => return Err("Raw counts.csv not found".into()),
                    _ => return Err(format!("Error opening file: {e}").into()),
                }
            }
        };

        let headers = match reader.headers() {
            Ok(h) => h.clone(),
            Err(e) => return Err(format!("Error extracting headers for Raw Counts: {e}").into()),
        };

        let mut records_processed = 0;

        let header_map: HashMap<&str, usize> = headers
            .iter()
            .enumerate()
            .map(|(i, h)| (h, i))
            .collect();

        let mut num_skipped_rows = 0;

        let date_format = format_description!("[year]-[month]-[day]");
        let allowed_directions = ["N", "E", "S", "W", "C", "J"];

        for result in reader.records() {
            let record = match result {
                Ok(r) => r,
                Err(e) => return Err(format!("Error extracting Raw Count row: {e}").into()),
            };

            let raw_count: RawCountCSV = match record.deserialize(Some(&headers)) {
                Ok(c) => c,
                Err(e) => {
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
                        num_skipped_rows += 1;
                        continue;
                    }

                    return Err(handle_count_csv_parse_error(Some(e), "Deserialization error", self.pool).await.into());
                }
            };

            let count_point = match data.count_points.get(&raw_count.count_point_id) {
                Some(cp) => cp,
                None => {
                    let message = format!("Count Point (ID: {}) doesn't exist in Count Points CSV", raw_count.count_point_id);
                    return Err(handle_count_csv_parse_error(None, &message, self.pool).await.into());
                }
            };

            let date = match Date::parse(&raw_count.count_date, &date_format) {
                Ok(d) => d,
                Err(e) => {
                    let message = format!("Error parsing date: {:?}. Raw date value: {}", e, &raw_count.count_date);
                    return Err(handle_count_csv_parse_error(None, &message, self.pool).await.into())
                }
            };

            let direction = raw_count.direction_of_travel.trim().to_uppercase();
            if !allowed_directions.contains(&direction.as_str()) {
                let message = format!("Invalid direction: {}. Allowed directions are: {:?}", direction, allowed_directions);
                return Err(handle_count_csv_parse_error(None, &message, self.pool).await.into());
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
                if let Err(e) = CountDB::insert_batch(self.pool, &batch).await {
                    return Err(format!("Error inserting batch of counts: {e}").into());
                }

                records_processed += batch.len() as u64;
                batch.clear();

                print!("\r   |-- {} records processed", records_processed.to_formatted_string(&Locale::en));
                std::io::stdout().flush().unwrap();
            }
        }

        if !batch.is_empty() {
            if let Err(e) = CountDB::insert_batch(self.pool, &batch).await {
                return Err(format!("Error inserting batch of counts: {e}").into());
            }

            records_processed += batch.len() as u64;

            print!("\r   |-- {} records processed", records_processed.to_formatted_string(&Locale::en));
        }

        println!("");
        if num_skipped_rows > 0 {
            println!("   |-- {} records skipped due to invalid count data", num_skipped_rows);
        }

        println!("");

        Ok(())
    }
}

async fn handle_count_csv_parse_error(e: Option<csv::Error>, message: &str, pool: &PgPool) -> String {
    let mut error_message = format!("Error parsing row: {message}. ");

    if let Some(e) = e {
        let info = e.position().unwrap();
        error_message.push_str(&format!("CSV error (row: {}): {:?}. ", info.line(), e));
    }

    error_message.push_str("Counts table wil be truncated. ");

    if let Err(e) = CountDB::truncate_all(pool).await {
        error_message.push_str(&format!("Error truncating traffic counts: {:?}", e));
    }

    return error_message;
}