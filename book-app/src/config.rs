use dotenv::dotenv;

#[derive(Debug)]
pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn load() -> Self {
        dotenv().ok();

        let database_url = std::env::var("DATABASE_URL").expect("Failed to load DATABASE_URL");

        Config { database_url }
    }
}
