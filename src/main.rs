use sqlx::{AssertSqlSafe, postgres::PgPoolOptions};
use clap::Parser;

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

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let config = Config {
        connection_string: args.conn,
        database_name: args.dbname,
        sql_create_database: include_str!("../sql/1-create-database.sql").into(),
        sql_create_traffic: include_str!("../sql/2-create-traffic.sql").into(),
        sql_create_accident: include_str!("../sql/3-create-accidents.sql").into(),
    };

    if let Err(e) = ensure_database_exists(&config).await {
        eprintln!("Error ensuring database exists: {:?}", e);
        std::process::exit(1);
    }

    let _ = PgPoolOptions::new()
        .max_connections(5)
        .connect( &format!("{}/{}", config.connection_string, config.database_name))
        .await
        .expect("Error connecting to database");
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