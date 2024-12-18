use anyhow::{Ok, Result};
use async_openai::{types::CreateEmbeddingRequestArgs, Client};
use std::fmt;

const MODEL: &str = "text-embedding-3-small";

pub type RawEmbedding = Vec<f32>;

/// Embed a search query
pub async fn embed_query(query: &str) -> Result<RawEmbedding> {
    let client = Client::new();

    // TODO: check to make sure input query is below the token limit
    let request = CreateEmbeddingRequestArgs::default()
        .model(MODEL)
        .input(query)
        .build()?;

    let response = client.embeddings().create(request).await?;

    // get the single embedding from the response
    Ok(response.data[0].clone().embedding)
}

// Embed documents in a batch
pub async fn embed_documents(docs: Vec<impl fmt::Display>) -> Result<Vec<RawEmbedding>> {
    let client = Client::new();
    let docs_owned: Vec<String> = docs.into_iter().map(|doc| doc.to_string()).collect();

    // TODO: check to make sure each string is below the token limit
    let request = CreateEmbeddingRequestArgs::default()
        .model(MODEL)
        .input(docs_owned)
        .build()?;

    let response = client.embeddings().create(request).await?;

    // get the raw embeddings from the response
    Ok(response
        .data
        .iter()
        .map(|embedding| embedding.embedding.clone())
        .collect())
}

// Extract the first quoted string from a search query
pub fn extract_first_quote(input: &str) -> Option<String> {
    let chars = input.chars();
    let mut inside_quote = false;
    let mut result = String::new();

    for c in chars {
        if c == '"' {
            if inside_quote {
                // End of the quoted string
                return Some(result);
            } else {
                // Start of a quoted string
                inside_quote = true;
            }
        } else if inside_quote {
            // Collect characters inside the quote
            result.push(c);
        }
    }

    // If no closing quote was found, return None
    None
}
