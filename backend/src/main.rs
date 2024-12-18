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

    #[arg(short, long, help = "Raw scraped courses file")]
    courses: String,

    #[arg(short, long, help = "Embedded courses file")]
    embedded: String,
}

#[derive(Serialize, Deserialize)]
struct Search {
    search: String,
}

#[debug_handler]
async fn search(
    State(db): State<Arc<VectorDB>>,
    Json(mut query): Json<Search>,
) -> Json<Vec<Course>> {
    // cut off the search query if it is too long
    if query.search.len() > 256 {
        query.search = query.search[..256].to_string();
    }

    // find the first text quoted in the search query
    let quoted = embed::extract_first_quote(&query.search);

    let search_embedding = embed::embed_query(&query.search).await.unwrap();
    let mut output = db
        .search_embedding(quoted, search_embedding, 20)
        .await
        .unwrap();

    // remove the embedded field from the output to save bandwidth
    for course in output.iter_mut() {
        course.embedding = None;
    }

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

    // block until redis is ready (handle connection error)
    loop {
        if db.is_ready().await {
            break;
        } else {
            println!("Waiting for redis to start...");
            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        }
    }

    // reindex the database if necessary
    if args.reindex || !db.is_populated().await? {
        println!("Reindexing database...");
        let courses = corpus::process_courses(&args.courses, &args.embedded).await?;
        db.reset().await?;
        db.populate_database(courses).await?;
        db.create_index().await?;
    }

    println!("Server running on port 8080");

    let comression_layer: CompressionLayer = CompressionLayer::new()
        .br(true)
        .deflate(true)
        .gzip(true)
        .zstd(true);

    let mut app = Router::new()
        .route("/search", post(search))
        .nest_service("/", ServeDir::new(args.frontend))
        .layer(comression_layer)
        .with_state(Arc::new(db));

    if args.local {
        let cors_layer = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(vec![Method::GET, Method::POST])
            .allow_headers(Any);
        app = app.layer(cors_layer);
    }

    // run app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
