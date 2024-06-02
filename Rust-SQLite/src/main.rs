use sqlx::{migrate::MigrateDatabase, sqlite::SqliteQueryResult, Sqlite, SqlitePool};
use std::result::Result;

async fn create_schema(db_url: &str) -> Result<SqliteQueryResult, sqlx::Error> {
    let pool = SqlitePool::connect(db_url).await?;

    let query = r#"
        PRAGMA foreign_keys = ON;

        CREATE TABLE IF NOT EXISTS settings (
            id          INTEGER PRIMARY KEY,
            description TEXT NOT NULL,
            created_at  DATETIME DEFAULT (datetime('now', 'localtime')),
            updated_at  DATETIME DEFAULT (datetime('now', 'localtime')),
            done        BOOLEAN NOT NULL DEFAULT FALSE
        );
        
        CREATE TABLE IF NOT EXISTS project (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            name        TEXT NOT NULL,
            created_at  DATETIME DEFAULT (datetime('now', 'localtime')),
            updated_at  DATETIME DEFAULT (datetime('now', 'localtime')),
            img_dir     TEXT NOT NULL,
            out_dir     TEXT NOT NULL,
            status      TEXT NOT NULL,
            settings_id INTEGER NOT NULL DEFAULT 1,

            FOREIGN KEY (settings_id) REFERENCES settings (id) ON UPDATE SET NULL ON DELETE SET NULL
        );"#;
    let result = sqlx::query(&query).execute(&pool).await;

    pool.close().await;

    result
}

#[async_std::main]
async fn main() {
    let db_url = String::from("sqlite://sqlite.db");
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        Sqlite::create_database(&db_url).await.unwrap();
        match create_schema(&db_url).await {
            Ok(_) => println!("Schema created successfully"),
            Err(e) => panic!("Error creating schema: {:?}", e),
        }
    }

    let instances = SqlitePool::connect(&db_url).await.unwrap();
    
    let query = "INSERT INTO settings (description) VALUES ($1);";
    let result = sqlx::query(&query)
        .bind("testing")
        .execute(&instances)
        .await;

    instances.close().await;
    println!("{:?}", result);
}
