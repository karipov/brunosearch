use anyhow::{anyhow, Result};
use axum::{extract::State, routing::post, Json, Router};
use axum_macros::debug_handler;
use dotenvy::dotenv;
use http::Method;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use subprocess::Exec;
use tower_http::{compression::CompressionLayer, cors::Any, cors::CorsLayer, services::ServeDir};

use course::Course;
use vector::VectorDB;

pub mod corpus;
pub mod course;
pub mod embed;
pub mod vector;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "Course search engine")]
struct Args {
    #[arg(short, long, help = "Specify whether deployment is local or not")]
    local: bool,

    #[arg(short, long, help = "Specifiy to reindex the database")]
    reindex: bool,

    #[arg(short, long, help = "Frontend static file directory")]
    frontend: String,

    #[arg(
        short,
        long,
        required_if_eq("reindex", "true"),
        help = "Raw scraped courses file"
    )]
    courses: Option<String>,

    #[arg(
        short,
        long,
        required_if_eq("reindex", "true"),
        help = "Embedded courses file"
    )]
    embedded: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Search {
    search: String,
}

#[debug_handler]
async fn search(State(db): State<Arc<VectorDB>>, Json(query): Json<Search>) -> Json<Vec<Course>> {
    // find the first text quoted in the search query
    let quoted = embed::extract_first_quote(&query.search);

    let search_embedding = embed::embed_query(&query.search).await.unwrap();
    let output = db
        .search_embedding(quoted, search_embedding, 20)
        .await
        .unwrap();

    Json(output)
}

#[tokio::main]
async fn main() -> Result<()> {
    // get the command line arguments
    let args = Args::parse();

    // start redis
    println!("Starting redis...");
    if args.local {
        println!("Loading environment variables...");
        dotenv().ok();

        Exec::cmd("docker")
            .arg("start")
            .arg("brunosearch-redis-1")
            .capture()
            .map_err(|_| {
                anyhow!("Failed to start redis, make sure brunosearch container is running")
            })?;
    } else {
        Exec::cmd("redis-server")
            .args(&["--loadmodule", "/opt/redis-stack/lib/redisearch.so"])
            .args(&["--loadmodule", "/opt/redis-stack/lib/rejson.so"])
            .args(&["--loadmodule", "/opt/redis-stack/lib/redisbloom.so"])
            .args(&["--port", "6379"])
            .args(&["--save", ""])
            .args(&["--daemonize", "yes"])
            .capture()
            .map_err(|_| anyhow!("Failed to start redis"))?;
    }

    println!("Connecting to redis...");
    let db = VectorDB::new()?;

    // delete the existing index and records
    if args.reindex {
        println!("Reindexing database...");
        let courses =
            corpus::process_courses(&args.courses.unwrap(), &args.embedded.unwrap()).await?;
        db.reset().await.unwrap();
        db.populate_database(courses).await.unwrap();
        db.create_index().await.unwrap();
    }

    println!("Server running on port 8080");
    // build application with a single route
    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_headers(Any);

    let comression_layer: CompressionLayer = CompressionLayer::new()
        .br(true)
        .deflate(true)
        .gzip(true)
        .zstd(true);

    let app = Router::new()
        .route("/search", post(search))
        .nest_service("/", ServeDir::new(args.frontend))
        .layer(cors_layer)
        .layer(comression_layer)
        .with_state(Arc::new(db));

    // run app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
