use std::collections::HashMap;

use sqlx::{AssertSqlSafe, Postgres, QueryBuilder};
use sqlx::postgres::PgPoolOptions;
use sqlx::postgres::types::PgPoint;
use sqlx::{FromRow, PgPool, postgres::PgRow};

use time::Date;

use crate::models::Config;

#[derive(FromRow, Debug)]
pub struct RoadCategoryDB {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub is_major: bool,
}

impl RoadCategoryDB {
    pub async fn load_from_db(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        load_records(pool, "
            SELECT id, name, description, is_major
            FROM traffic.road_categories
            ORDER BY name ASC
        ")
        .await
    }

    pub async fn load_from_db_hashmap(pool: &PgPool) -> Result<HashMap<String, Self>, sqlx::Error> {
        let records = Self::load_from_db(pool).await?;

        let mut road_categories: HashMap<String, Self> = HashMap::new();
        for record in records {
            road_categories.insert(record.name.clone(), record);
        }

        return Ok(road_categories);
    }

    pub async fn insert(pool: &PgPool, category_name: &str, description: &str) -> Result<Self, sqlx::Error> {
        sqlx::query_as("
            INSERT INTO traffic.road_categories(name, description)
            VALUES($1, $2)
            RETURNING *;
        ")
        .bind(category_name)
        .bind(description)
        .fetch_one(pool)
        .await
    }
}

#[derive(FromRow, Debug)]
pub struct RegionDB {
    pub id: i32,
    pub name: String,
    pub ons_code: String,
}

impl RegionDB {
    pub async fn load_from_db(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        load_records(pool, "
            SELECT id, name, ons_code
            FROM traffic.regions
            ORDER BY id ASC;
        ")
        .await
    }

    pub async fn load_from_db_hashmap(pool: &PgPool) -> Result<HashMap<i32, Self>, sqlx::Error> {
        let records = Self::load_from_db(pool).await?;

        let mut regions: HashMap<i32, Self> = HashMap::new();
        for record in records {
            regions.insert(record.id, record);
        }

        return Ok(regions);
    }

    pub async fn insert(pool: &PgPool, id: i32, name: &str, ons_code: &str) -> Result<Self, sqlx::Error> {
        sqlx::query_as("
            INSERT INTO traffic.regions(id, name, ons_code)
            VALUES($1, $2, $3)
            RETURNING *;
        ")
        .bind(id)
        .bind(name)
        .bind(ons_code)
        .fetch_one(pool)
        .await
    }
}

#[derive(FromRow, Debug)]
pub struct LocalAuthorityDB {
    pub id: i32,
    pub region_id: i32,
    pub name: String,
    pub ons_code: String,
}

impl LocalAuthorityDB {
    pub async fn load_from_db(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        load_records(pool, "
            SELECT id, region_id, name, ons_code
            FROM traffic.local_authorities
            ORDER BY id ASC;
        ")
        .await
    }

    pub async fn load_from_db_hashmap(pool: &PgPool) -> Result<HashMap<i32, Self>, sqlx::Error> {
        let records = Self::load_from_db(pool).await?;

        let mut local_authorities: HashMap<i32, Self> = HashMap::new();
        for record in records {
            local_authorities.insert(record.id, record);
        }

        return Ok(local_authorities);
    }

    pub async fn insert(pool: &PgPool, id: i32, region_id: i32, name: &str, ons_code: &str) -> Result<Self, sqlx::Error> {
        sqlx::query_as("
            INSERT INTO traffic.local_authorities(id, region_id, name, ons_code)
            VALUES($1, $2, $3, $4)
            RETURNING *;
        ")
        .bind(id)
        .bind(region_id)
        .bind(name)
        .bind(ons_code)
        .fetch_one(pool)
        .await
    }
}

#[derive(FromRow, Debug)]
pub struct CountPointDB {
    pub id: i32,
    pub local_authority_id: i32,
    pub road_category_id: i32,
    pub road_name: String,
    pub location: PgPoint,
}

impl CountPointDB {
    pub async fn load_from_db(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        load_records(pool, "
            SELECT id, local_authority_id, road_category_id, road_name, location
            FROM traffic.count_points
            ORDER BY id ASC;
        ")
        .await
    }

    pub async fn load_from_db_hashmap(pool: &PgPool) -> Result<HashMap<i32, Self>, sqlx::Error> {
        let records = Self::load_from_db(pool).await?;

        let mut count_points: HashMap<i32, Self> = HashMap::new();
        for record in records {
            count_points.insert(record.id, record);
        }

        return Ok(count_points);
    }

    pub async fn insert_batch(pool: &PgPool, rows: &[Self]) -> Result<(), sqlx::Error> {
        let mut query = QueryBuilder::<Postgres>::new("
            INSERT INTO traffic.count_points
            (
                id,
                local_authority_id,
                road_category_id,
                road_name,
                location
            )
        ");

        query.push_values(rows, |mut values, row| {
            values
                .push_bind(row.id)
                .push_bind(row.local_authority_id)
                .push_bind(row.road_category_id)
                .push_bind(&row.road_name)
                .push_bind(&row.location);
        });

        query.build().execute(pool).await?;

        Ok(())
    }
}


async fn load_records<T>(pool: &PgPool, query: &'static str) -> Result<Vec<T>, sqlx::Error>
where
    for<'r> T: FromRow<'r, PgRow> + Send + Unpin,
{
    sqlx::query_as::<_, T>(query)
        .fetch_all(pool)
        .await
}

#[derive(Debug, Clone)]
pub struct CountDB {
    pub count_point_id: i32,
    pub date: time::Date,
    pub hour: i16,
    pub direction: String,
    pub bicycles: i32,
    pub motorcycles: i32,
    pub cars: i32,
    pub buses: i32,
    pub lgvs: i32,
    pub hgvs: i32,
}

impl CountDB {
    // Counts table has a unique constraint on "count_point_id, date, hour, direction", a very small number of
    // records from the CSV violate this constraint. For these records I just want the total vehicle counts to
    // be merged into a single DB record. So if there's "123, 2025-01-01, 7, 'N', Cars = 20", and another
    // "123, 2025-01-01, 7, 'N', Cars = 15", there should be just one record "123, 2025-01-01, 7, 'N', Cars = 35"
    // The "ON CONFLICT ON CONSTRAINT counts_unique_observation" SQL below does this, but it won't work for
    // INSERTS that are batched together, so this function is needed to check for duplicates within a single
    // batch. Batches are about 2000 at a time, so shouldn't impact import performance too much.
    fn merge_duplicate_rows(rows: &[Self]) -> Vec<CountDB> {
        let mut merged: HashMap<(i32, Date, i16, String), CountDB> = HashMap::with_capacity(rows.len());

        for row in rows {
            let key = (
                row.count_point_id,
                row.date,
                row.hour,
                row.direction.clone(),
            );

            merged
                .entry(key)
                .and_modify(|existing| {
                    existing.bicycles += row.bicycles;
                    existing.motorcycles += row.motorcycles;
                    existing.cars += row.cars;
                    existing.buses += row.buses;
                    existing.lgvs += row.lgvs;
                    existing.hgvs += row.hgvs;
                })
                .or_insert_with(|| row.clone());
        }

        merged.into_values().collect()
    }

    pub async fn insert_batch(pool: &PgPool, rows: &[Self]) -> Result<(), sqlx::Error> {
        let rows = CountDB::merge_duplicate_rows(rows);

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

        query.push("
            ON CONFLICT ON CONSTRAINT counts_unique_observation
            DO UPDATE SET
                bicycles    = traffic.counts.bicycles    + EXCLUDED.bicycles,
                motorcycles = traffic.counts.motorcycles + EXCLUDED.motorcycles,
                cars        = traffic.counts.cars        + EXCLUDED.cars,
                buses       = traffic.counts.buses       + EXCLUDED.buses,
                lgvs        = traffic.counts.lgvs        + EXCLUDED.lgvs,
                hgvs        = traffic.counts.hgvs        + EXCLUDED.hgvs;
        ");

        query.build().execute(pool).await?;

        return Ok(());
    }

    pub async fn truncate_all(pool: &PgPool) -> Result<(), sqlx::Error> {
        let result = sqlx::query("TRUNCATE TABLE traffic.counts;")
            .execute(pool)
            .await;

        if let Err(e) = result {
            return Err(e);
        }

        return Ok(());
    }
}

pub async fn ensure_database_exists(config: &Config) -> Result<(), sqlx::Error> {
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
        let sql_create_database = config.sql_create_database.clone()
            .replace("####DATEBASE_NAME####", &config.database_name);
        sqlx::query(AssertSqlSafe(sql_create_database))
            .execute(&pool)
            .await?;
    }

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

    Ok(())
}