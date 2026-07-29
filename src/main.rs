pub mod database;
pub mod csv_utils;
pub mod models;
pub mod importer;

use std::io::Write;
use std::time::{Instant};
use clap::Parser;
use sqlx::{postgres::PgPoolOptions};

use crate::models::{Config};
use crate::importer::traffic::TrafficImporter;

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
    println!("version: 0.3.0");
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
    println!("");

    // connect to database
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect( &format!("{}/{}", config.connection_string, config.database_name))
        .await
        .expect("Error connecting to database");

    // run traffic import
    let mut traffic_importer = TrafficImporter::new(&pool);
    if let Err(e) = traffic_importer.run().await {
        eprintln!("{e}");
        std::process::exit(1);
    }

    println!("");
    println!("Finished! Took {:?}", timer.elapsed());
}