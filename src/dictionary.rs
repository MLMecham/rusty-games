// API key: QOwqV6Hb558kFN/2mikqBQ==Dd5jt0l2H2BCxARW
// website: https://www.api-ninjas.com/examples/basic-web-app

use std::error::Error;
use serde_json::Value;
use rand::Rng; // For generating random numbers

pub async fn get_word() -> Result<String, Box<dyn Error>> {
    // Replace with your API Ninjas API key
    let api_key = "QOwqV6Hb558kFN/2mikqBQ==Dd5jt0l2H2BCxARW";

    // Call the function to get a random word
    // if let Err(e) = fetch_random_word(api_key).await {
    //     eprintln!("Error: {}", e);
    // }

    fetch_random_word(api_key).await
}

// Function to fetch a random word from API Ninjas
async fn fetch_random_word(api_key: &str) -> Result<String, Box<dyn Error>> {
    let url = "https://api.api-ninjas.com/v1/randomword";

    // Create a reqwest client
    let client = reqwest::Client::new();

    // Make the GET request with the API key in the header
    let response = client
        .get(url)
        .header("X-Api-Key", api_key)
        .send()
        .await?;

    if response.status() != reqwest::StatusCode::OK {
        return Ok(random_word_from_bank());
    }

    // Extract the body text
    let body: String = response.text().await?;

    // Parse the JSON response
    let parsed: Value = serde_json::from_str(&body)?;

    // Extract and print the random word
    // format:  {"word": ["stanza"]}; to get it: array.get(0)
    if let Some(word_array) = parsed["word"].as_array() {
        if let Some(random_word) = word_array.get(0).and_then(|w| w.as_str()){
            Ok(random_word.to_string())
        }else {
            Ok(random_word_from_bank())
        }
    } else {
        Err("Can't retrieve word from API nor word bank: {}".into())
    }

    
}

// Function to return a random word from the word bank
fn random_word_from_bank() -> String {
    let word_bank = vec![
        "acquire", "beneficial", "chaos", "delicate", "emerge", "fluctuate", "graceful", 
        "harmony", "illuminate", "justify", "keen", "leisure", "magnify", "notable", 
        "obstacle", "prosper", "quaint", "resilient", "sustain", "tenacious", "unique", 
        "vivid", "wholesome", "zealous", "adapt", "boundary", "compose", "diminish", 
        "enrich", "fortify", "genuine", "heritage", "innovate", "jubilant", "kinship", 
        "landscape", "meticulous", "nurture", "optimistic", "perceive", "radiant", 
        "sacrifice", "tolerate", "uplift", "versatile", "wander", "yearn", "zephyr", 
        "aspire", "balance", "conquer"
    ];

    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..word_bank.len());
    word_bank[random_index].to_string()
}