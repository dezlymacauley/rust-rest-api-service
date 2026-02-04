// For reading environment variables
// I may use `dotenv` later but this is fine for now
use std::env;

use sqlx::{
    // Brings the connection `Pool<T>` Struct data type into scope
    // `T` is a generic type for the database used by this program.
    Pool, 

    //Brings the `Postgres` database into scope so that the data type:
    // Pool<Postgres> can be used.
    Postgres, 

    // Brings the `postgres` crate (aka the Postgres database driver) 
    // into scope. This is a collection of SQLx tools that are specifically
    // for working with Postgres databases.
    postgres::{
        // PgPoolOptions is just a type alias for the generic struct:
        // PoolOptions<T>.
        // `T` is database type (Postgres, MySql, Sqlite, etc.)
        PgPoolOptions
    }

};

// This is required to make the `main()` function asynchronous
#[tokio::main]
async fn main() {
    // Handle this gracefully later.
    let db_url: String = env::var("DATABASE_URL")
        .expect("Error: The environment variable DATABASE_URL is not set");

    // Remove this later
    println!("DATABASE_URL: {db_url}");

    // `new()` creates a builder for the Postgres connection pool
    let pool: Pool<Postgres> = PgPoolOptions::new()
        // This is an async method that will attempt to add a database
        // to the connection pool
        // `.await` runs the async method and waits for it to complete,
        // without blocking the thread, 
        // allowing other async tasks to run in the meantime.
        .connect(&db_url).await
        .expect("Error: Failed to connect to the database");

    // migrate()! is a macro that produces a migration manager object
    // sqlx::migrate!("");
}
