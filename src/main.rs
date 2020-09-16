use std::env;

fn main() {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap_or("".to_string());
    println!("DATABASE_URL: {}", database_url);
}
