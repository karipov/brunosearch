use anyhow::Result;

use crate::course::Course;
use crate::embed::embed_documents;
use crate::embed::RawEmbedding;

// process course files from json and embed them
pub async fn process_courses(raw_path: &str, embedded_path: &str) -> Result<Vec<Course>> {
    Ok(match read_courses(embedded_path) {
        Ok(courses) => {
            // using the embdedded courses
            courses
        }
        Err(_) => {
            // embedding the courses
            let courses = read_courses(raw_path)?;
            let embeddings = embed_documents(courses.clone()).await?;
            let courses = embed_courses(courses, embeddings);
            write_courses(embedded_path, courses.clone())?;
            courses
        }
    })
}

// Calculates the embeddings for a vector of courses and adds them to the objects
pub fn embed_courses(courses: Vec<Course>, embeddings: Vec<RawEmbedding>) -> Vec<Course> {
    courses
        .into_iter()
        .zip(embeddings)
        .map(|(mut course, embedding)| {
            course.embedding = Some(embedding);
            course
        })
        .collect()
}

// Reads the courses from a json file
pub fn read_courses(file_path: &str) -> Result<Vec<Course>> {
    let file = std::fs::File::open(file_path)?;
    let courses: Vec<Course> = serde_json::from_reader(file)?;
    Ok(courses)
}

// Writes the courses to a json file
pub fn write_courses(file_path: &str, courses: Vec<Course>) -> Result<()> {
    let file = std::fs::File::create(file_path)?;
    serde_json::to_writer_pretty(file, &courses)?;
    Ok(())
}
