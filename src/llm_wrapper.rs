use futures::StreamExt;
use reqwest::Client;
use serde_json::{json, Value};
use std::{error::Error as StdError, fmt};

#[derive(Clone)]
pub struct Llm {
    url: &'static str,
    model: &'static str,
    client: Client,
}

#[derive(Debug)]
struct CustomError(String);

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl StdError for CustomError {}

impl Llm {
    fn format_response(text: &str) -> String {
        // Replace escape sequences with actual symbols and format the string
        let formatted_text = text
            .replace("\\n", "\n") // Replace `\n` with actual newlines
            .replace("\\\"", "\"") // Replace escaped quotes
            .replace("\\\\", "\\") // Replace double backslashes with a single one
            .replace("```", "\n```"); // Ensure proper line breaks before code blocks

        formatted_text
    }

    pub async fn send_prompt(&self, prompt: &str) -> String {
        let json_body = json!({
            "model": &self.model,
            "prompt": prompt
        });

        // Send a POST request with the JSON body
        let response = self
            .client
            .post(self.url)
            .json(&json_body) // Send the JSON body
            .send()
            .await;

        // Check if the request was successful
        let response = match response {
            Ok(rep) => rep,
            Err(e) => {
                eprintln!("Request failed with error: {e:?}");
                return String::new();
            }
        };

        let mut result_builder = String::new();
        if response.status().is_success() {
            // Get the response body as a stream of bytes
            let mut stream = response.bytes_stream();

            // Process each chunk as it arrives
            while let Some(chunk_result) = stream.next().await {
                match chunk_result {
                    Ok(chunk) => {
                        // Convert the chunk into a valid JSON string
                        if let Ok(json_chunk) = serde_json::from_slice::<Value>(&chunk) {
                            let parsed: Value = serde_json::from_str(&json_chunk.to_string())
                                .expect("Failed to parse JSON");
                            let data = parsed
                                .get("response")
                                .expect("No `data` field found")
                                .to_string();

                            let trimmed_data = &data[1..data.len() - 1];

                            result_builder += &Self::format_response(trimmed_data);
                        } else {
                            println!("Failed to parse a chunk of the response");
                        }
                    }
                    Err(e) => {
                        println!("Error receiving chunk: {e}");
                    }
                }
            }
        } else {
            println!("Request failed with status: {}", response.status());
        }

        result_builder
    }

    pub fn new(url: &'static str, model: &'static str) -> Self {
        let client = Client::new();
        Self { url, model, client }
    }
}
