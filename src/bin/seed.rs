use std::fs::{read_dir, read_to_string};

use sqlx::{PgPool, Pool, Postgres};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = PgPool::connect(&dotenvy::var("DATABASE_URL")?).await?;
    
    run_sql_files(&pool).await?;

    println!("Seed data loaded successfully");
    Ok(())
}

async fn run_sql_files(pool: &Pool<Postgres>) -> anyhow::Result<()> {
    for file in read_dir("./database/seed/")? {
        let sql = read_to_string(format!("{}", file?.path().display()))?;
        sqlx::raw_sql(&sql).execute(pool).await?;
    }
    
    Ok(())
}