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

use axum::{
    Router, 
    routing::{
        get, 
        post
    }
};

use lib_dot_rs::event_handlers::*;

// This is required to make the `main()` function asynchronous
#[tokio::main]
async fn main() {
    //_________________________________________________________________________

    // SECTION: Postgres Setup

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
        // It takes the contents of those SQL files inside 
        // the `database/migrations` directory and makes them a part of the
        // binary of this program, so that you don't need to ship the .sql
        // files separately when you deploy.

    // `migrate!()` is a macro that produces 
    // a migration manager (`Migrator`) object.
    // It takes the SQL files inside the `database/migrations` directory
    // and embeds their contents into the binary, 
    // so you don't need to ship the `.sql` files separately when deploying.
    sqlx::migrate!("database/migrations")
        // This will run the embedded SQL commands
        // `.run()` is asynchronous so await is needed.
        .run(&pool).await
        .expect("Error: Migrations failed");

    //_________________________________________________________________________
    
    // SECTION: Axum Setup (The Server)

    // I am using "0.0.0.0:8000" because I plan to use Docker later
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await
        .expect("Error: Could not bind to port");

    // TIP: Ignore the warning for `app` that says type annotations needed.
    // This will be resolved when you use the  `axum:server()` function

    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user).get(list_users))
        .route(
            "users/{id}", 
            get(get_user).put(update_user).delete(delete_user)
        )
        .with_state(pool);

    println!("✅ Server is running:");

    // Make this dynamic later.
    println!("0.0.0.0:8000");

    axum::serve(listener, app).await
        .expect("Error: Failed the start the Axum server");

    //_________________________________________________________________________
}
