# Rust Web Scraper (Mongo-Driven Configurable Scraper)

## Overview

This application is a highly flexible and modular **web + API scraper**, built in Rust, driven entirely by configuration stored in a MongoDB database.

Instead of hardcoding scraping logic, it pulls structured scraper definitions from a **MongoDB `configuration` collection**, then dynamically executes those scraping jobs based on type (`web` or `api`).

### Use Cases

- News and forum monitoring  
- API data fetching  
- Categorization and cataloging into MongoDB  
- Easily expandable with new source types  

---

## Architecture

- **Rust-based**, fully async (via `tokio`)
- **MongoDB** is the central control point:
  - Stores scraper configuration
  - Stores categorized scraped data
- **Modular structure**:
  - `lib/mongodb.rs` → Mongo factory
  - `lib/config.rs` → Configuration schema
  - `utils/logger.rs` → Runtime logging with levels
- **Types of sources**:
  - `api` — JSON-based HTTP APIs
  - `web` — HTML scraping via DOM selectors

---

## MongoDB Schema

### Collection: `web_scraper.configuration`

```json
{
  "_id": ObjectId,
  "sources": [
    {
      "_id": "reddit_news",
      "type": "web",
      "host": "https://www.reddit.com",
      "path": "/r/news",
      "dom_selectors": [
        { "method": "class", "value": "Post" },
        { "method": "id", "value": "siteTable" }
      ]
    },
    {
      "_id": "market_api",
      "type": "api",
      "host": "https://api.something.com",
      "path": "/api/v3/stocks/markets",
      "method": "GET",
      "response_property": "name"
    }
  ]
}
```

---

### Collection: `web_scraper.categorized_data`

All scraped + categorized results get stored here.

```json
{
  "_id": ObjectId,
  "source_id": "reddit_news",
  "title": "Post title",
  "category": "News",
  "url": "https://reddit.com/post/123"
}
```

---

## ⚙️ `.env` Example

```env
MONGO_URI=mongodb://localhost:27017
MONGO_DB_NAME=web_scraper
MONGO_CONFIG_COLLECTION=configuration
MONGO_DATA_COLLECTION=categorized_data
LOG_LEVEL=debug
```

---

## 🚀 Running the Scraper

1. **Install dependencies**

```bash
cargo build
```

2. **Ensure MongoDB is running**, and the `configuration` collection contains your configs.

3. **Run the scraper**

```bash
cargo run
```

It will:
- Read configs from `MONGO_CONFIG_COLLECTION`
- Scrape each source based on its type
- Store categorized output into `MONGO_DATA_COLLECTION`

---

## 💡 TODO (Future Enhancements)

- Add CLI options for selecting config by ID
- Support for pagination and rate limiting
- Exporters (CSV/JSON)
- Scheduler (cron-style or queue-based)
- Webhook triggers
