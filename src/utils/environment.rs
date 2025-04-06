pub fn get_scrape_interval_minutes() -> u64 {
    std::env::var("SCRAPE_INTERVAL_MINUTES")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(5) // default to 5 minutes if missing or invalid
}