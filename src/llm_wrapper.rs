use std::error::Error;

use futures::StreamExt;
use log::info;
use reqwest::Client;
use serde_json::{json, Value};

#[derive(Clone)]
pub struct Llm {
    client: Client,
}

static URL: &str = "http://cygnus:11434/api/generate";
static BACKUP_URL: &str = "http://localhost:11434/api/generate";
static MODEL: &str = "llama3.2";

impl Llm {
    fn format_response(text: &str) -> String {
        // Replace escape sequences with actual symbols and format the string
        text.replace("\\n", "\n")
            .replace("\\\"", "\"")
            .replace("\\\\", "\\")
            .replace("```", "\n```")
    }

    pub async fn send_prompt(&mut self, prompt: &str) -> Result<String, Box<dyn Error>> {
        // let prompt_context_builder =
        //     "Here is context of this user. Base the following prompt on this: \'".to_owned()
        //         + &self.prompt_context
        //         + "\n"
        //         + "Do not mention this context on your following answers\n"
        //         + "Prompt: "
        //         + prompt;

        let json_body = json!({
            "model": &MODEL,
            "prompt": prompt
        });

        // Send a POST request with the JSON body
        let response = if let Ok(res) = self
            .client
            .post(URL)
            .json(&json_body) // Send the JSON body
            .send()
            .await
        {
            res
        } else {
            info!("Primary URL is unavailable, using localhost llm");
            self.client
                .post(BACKUP_URL)
                .json(&json_body) // Send the JSON body
                .send()
                .await?
        };

        let mut result_builder = String::new();
        if response.status().is_success() {
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

        Ok(result_builder)
    }

    pub fn new() -> Self {
        let client = Client::new();
        Self { client }
    }
}
