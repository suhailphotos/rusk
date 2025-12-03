use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct Config {
    databases: Databases,
}

#[derive(Debug, Deserialize)]
struct Databases {
    notion: NotionConfig,
    supabase: SupabaseConfig,
    postgres: PostgresConfig,
}

#[derive(Debug, Deserialize)]
struct NotionConfig {
    api_key: String,
    database_id: String,
}

#[derive(Debug, Deserialize)]
struct SupabaseConfig {
    url: String,
    anon_key: String,
}

#[derive(Debug, Deserialize)]
struct PostgresConfig {
    host: String,
    port: u16,
    user: String,
    password: String,
    dbname: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&contents)?;

    // Access every field at least once so there are no dead_code warnings.
    println!("Notion API key:      {}", config.databases.notion.api_key);
    println!("Notion database ID:  {}", config.databases.notion.database_id);

    println!("Supabase URL:        {}", config.databases.supabase.url);
    println!("Supabase anon key:   {}", config.databases.supabase.anon_key);

    println!(
        "Postgres: host={} port={} user={} db={} password={}",
        config.databases.postgres.host,
        config.databases.postgres.port,
        config.databases.postgres.user,
        config.databases.postgres.dbname,
        config.databases.postgres.password,
    );

    Ok(())
}
