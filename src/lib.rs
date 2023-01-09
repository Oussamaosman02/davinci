//! Use the davinci model from the OpenAI API
//!
//! This library provides a function for asking questions to the OpenAI Davinci model and getting a response.
//! # davinci
//! `davinci` is the main function, and it has 4 parameters:
//! * `api_key` -> String - This is the OpenAi api key.
//!     It can be obtained [here](https://beta.openai.com/account/api-keys)
//! * `context` -> String - The context for the question.
//! * `question` -> String - The question or phrase to ask the model.
//! * `tokens` -> i32 - The maximum number of tokens to use in the response.
//!
//! ## `context` and `question`
//! The `context` and `question` are the prompt for the model.
//! A prompt is a text string given to a model as input that gives the model a specific task to perform.
//!
//! Providing good and strong context to the model
//! (such as by giving a few high-quality examples of desired behavior prior to the new input)
//! can make it easier to obtain better and desired outputs.
//!
//! ## `tokens`
//! Tokens is the max. tokens to be generated (counting the prompt) by a model.
//!
//! The GPT family of models process text using tokens, which are common sequences of characters found in text.
//! The models understand the statistical relationships between these tokens, and excel at producing the next token in a sequence of tokens.
//!
//!
//! Token generally corresponds to ~4 characters of text for common English text.
//! This translates to roughly ¾ of a word (so 100 tokens ~= 75 words).
//!
//! Another thing to keep in mind, is that the `tokens` highest value is 2048 (4096 in new models).
//!
//! One way to know the number of tokens your prompt has is using [this site](https://beta.openai.com/tokenizer)
//!
//! ## Example of usage
//! In this quick example we use davinci to find a answer to user's question.
//!
//! ```
//! use davinci::davinci;
//! use std::io;
//!
//! fn main() {
//!     let api: String = String::from("vj-JZkjskhdksKXOlncknjckukNKKnkJNKJNkNKNk");
//!     let max_tokens: i32 = 100;
//!     let context: String =
//!         String::from("The following is a conversation with an AI assistant. The assistant is helpful, creative, clever, and very friendly.\n\nHuman: Hello, who are you?\nAI: I am an AI created by OpenAI. How can I help you today?");
//!     println!("What is your question?");
//!     // Reading the user input
//!     let mut question: String = String::new();
//!     io::stdin()
//!         .read_line(&mut question)
//!         .expect("Error, you have to write something!");
//!     let response: String = match davinci(api, context, question, max_tokens) {
//!         Ok(res) => res,
//!         Err(error) => error.to_string(),
//!     };
//!     println!("{}", response);
//! }
//! ```
//!
use reqwest::{Client, Error, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Parameters {
    model: String,
    prompt: String,
    temperature: f64,
    max_tokens: i32,
    top_p: u8,
    frequency_penalty: f64,
    presence_penalty: f64,
    stop: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Choice {
    text: String,
    index: u8,
    logprobs: Option<i32>,
    finish_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Usage {
    prompt_tokens: i32,
    completion_tokens: i32,
    total_tokens: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAIResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<Choice>,
    usage: Usage,
}
#[tokio::main]
/// # Parameters
///
/// * `api_key` - The OpenAI API key.
///     This must be well written, as it will throw an error if not.
/// * `context` - The context for the question.
///     The context is important for good responses as it tells the model how it should be it's behavior.
/// * `question` - The question or phrase to ask the model.
/// * `tokens` - The maximum number of tokens to use in the response.
///
/// # Returns
///
/// Returns the model's response as a Ok(String) or an Error.
///
pub async fn davinci(
    api_key: String,
    context: String,
    question: String,
    tokens: i32,
) -> Result<String, Error> {
    let bearer = String::from("Bearer ") + &api_key;

    let resp: String = format!("{}.\nH: {}.\nIA:", context, question);
    let prompt = Parameters {
        model: String::from("text-davinci-003"),
        prompt: resp,
        temperature: 0.9,
        max_tokens: tokens,
        top_p: 1,
        frequency_penalty: 0.0,
        presence_penalty: 0.6,
        stop: vec![String::from("\n")],
    };

    let client = Client::new();
    let resp: Response = client
        .post("https://api.openai.com/v1/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", bearer)
        .json(&prompt)
        .send()
        .await
        .expect("Error while getting the response");

    let openai_response: OpenAIResponse = resp
        .json()
        .await
        .expect("Error while generating the response");

    let formatted_response = format!("{}", openai_response.choices[0].text);
    return Ok(formatted_response);
}
