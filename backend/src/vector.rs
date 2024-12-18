use std::env;
use redis::{Client, pipe};
use anyhow::Result;

use crate::course::Course;

const REDIS_INDEX: &str = "idx:course_vss";

pub struct VectorDB {
    client: Client,
}

impl VectorDB {
    /// Create a new VectorDB instance
    pub fn new() -> Result<Self> {
        let client = Client::open(env::var("REDIS_URL")?)?;

        Ok(Self { client })
    }

    /// reset the database
    pub async fn reset(&self) -> Result<()> {
        let mut con = self.client.get_multiplexed_async_connection().await?;
        redis::cmd("FLUSHALL").exec_async(&mut con).await?;
        Ok(())
    }

    #[rustfmt::skip]
    pub async fn search_embedding(&self,
        quoted: Option<String>,
        embedding: Vec<f32>,
        num_results: usize) -> Result<Vec<Course>> {
        let mut con = self.client.get_multiplexed_async_connection().await?;

        let raw_types: Vec<u8> = embedding.iter().flat_map(|&f| f.to_ne_bytes()).collect();
        let pre_filter = quoted.unwrap_or("*".to_string());

        // search for the most similar courses, returning the course keys
        
        let results: Vec<String> = redis::cmd("FT.SEARCH").arg(REDIS_INDEX)
            .arg(format!("({})=>[KNN {} @embedding $query_vector AS score]", pre_filter, num_results))
            .arg("PARAMS").arg(2).arg("query_vector").arg(&raw_types)
            .arg("SORTBY").arg("score").arg("ASC")
            .arg("NOCONTENT")
            .arg("LIMIT").arg(0).arg(num_results)
            .arg("DIALECT").arg(4)
            .query_async(&mut con).await?;
            
        dbg!(&results);

        // build a pipeline to get the course data
        let mut pipeline = pipe();
        for x in results.into_iter().skip(1) { // note: first element is number of results
            pipeline.json_get(x, "$")?;
        }

        let courses = pipeline.query_async(&mut con).await?;

        Ok(courses)
    }

    /// Create a vector similarity search index in Redis
    #[rustfmt::skip]
    pub async fn create_index(&self) -> Result<()> {
        let mut con = self.client.get_multiplexed_async_connection().await?;

        // check if the index already exists and drop it if it does
        if redis::cmd("FT.INFO").arg(REDIS_INDEX).exec_async(&mut con).await.is_ok() {
            redis::cmd("FT.DROPINDEX").arg(REDIS_INDEX).exec_async(&mut con).await?;
        }
        
        // create the index (don't need full department name in index)
        
        redis::cmd("FT.CREATE").arg(REDIS_INDEX).arg("ON").arg("JSON")
            .arg("PREFIX").arg(1).arg("courses:")
            .arg("SCHEMA")
                .arg("$.department_short").arg("AS").arg("department_short").arg("TEXT").arg("NOSTEM")
                .arg("$.code")            .arg("AS").arg("code")            .arg("TEXT").arg("NOSTEM")
                .arg("$.title")           .arg("AS").arg("title")           .arg("TEXT")
                .arg("$.professor")       .arg("AS").arg("professor")       .arg("TEXT").arg("NOSTEM")
                .arg("$.time")            .arg("AS").arg("time")            .arg("TEXT").arg("NOSTEM")
                .arg("$.description")     .arg("AS").arg("description")     .arg("TEXT")

                .arg("$.writ")            .arg("AS").arg("writ")            .arg("TAG")
                .arg("$.soph")            .arg("AS").arg("soph")            .arg("TAG")
                .arg("$.fys")             .arg("AS").arg("fys")             .arg("TAG")
                .arg("$.rpp")             .arg("AS").arg("rpp")             .arg("TAG")
                
                .arg("$.embedding")       .arg("AS").arg("embedding")       .arg("VECTOR")
                    .arg("FLAT").arg(6)
                    .arg("TYPE").arg("FLOAT32")
                    .arg("DIM").arg(1536)
                    .arg("DISTANCE_METRIC").arg("COSINE")
            .exec_async(&mut con).await?;

        Ok(())
    }

    /// Populate the vector similarity search index with courses
    pub async fn populate_database(&self, courses: Vec<Course>) -> Result<()> {
        let mut con = self.client.get_multiplexed_async_connection().await?;
        let mut pipeline = pipe();

        for course in courses {
            pipeline.json_set(
                format!("courses:{}:{}", course.department_short, course.code),
                "$",
                &course
            )?;
        }

        pipeline.exec_async(&mut con).await?;

        Ok(())
    }

}