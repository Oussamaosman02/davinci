# Davinci

> A crate for simply using davinci model from OpenAi API.

This library provides a function for asking questions to the OpenAI Davinci model and getting a response.

## Dependencies

This library use 3 unique dependencies:

- `reqwest` : for making the API call -> 144 kB
- `tokio` : for manage async and await -> 625 kB
- `serde` : for parsing the OpenAi api response -> 77.1 kB

## `fn davinci`

`davinci` is the main function, and it has 4 parameters:

- `api_key` -> String - This is the OpenAi api key.
  It can be obtained [here](https://beta.openai.com/account/api-keys)
- `context` -> String - The context for the question.
- `question` -> String - The question or phrase to ask the model.
- `tokens` -> i32 - The maximum number of tokens to use in the response.

### `context` and `question`

The `context` and `question` are the prompt for the model.
A prompt is a text string given to a model as input that gives the model a specific task to perform.

Providing good and strong context to the model
(such as by giving a few high-quality examples of desired behavior prior to the new input)
can make it easier to obtain better and desired outputs.

### `tokens`

Tokens is the max. tokens to be generated (counting the prompt) by a model.

The GPT family of models process text using tokens, which are common sequences of characters found in text.
The models understand the statistical relationships between these tokens, and excel at producing the next token in a sequence of tokens.

Token generally corresponds to ~4 characters of text for common English text.
This translates to roughly ¾ of a word (so 100 tokens ~= 75 words).

Another thing to keep in mind, is that the `tokens` highest value is 2048 (4096 in new models).

One way to know the number of tokens your prompt has is using [this site](https://beta.openai.com/tokenizer)

## Example of usage

In this quick example we use davinci to find a answer to user's question.

```rust
use davinci::davinci;
use std::io;

fn main() {
    let api: String = String::from("vj-JZkjskhdksKXOlncknjckukNKKnkJNKJNkNKNk");

    let max_tokens: i32 = 100;

    let context: String =
        String::from("The following is a conversation with an AI assistant. The assistant is helpful, creative, clever, and very friendly.\n\nHuman: Hello, who are you?\nAI: I am an AI created by OpenAI. How can I help you today?");

    println!("What is your question?");

    // Reading the user input
    let mut question: String = String::new();

    io::stdin()
        .read_line(&mut question)
        .expect("Error, you have to write something!");

    let response: String = match davinci(api, context, question, max_tokens) {
        Ok(res) => res,
        Err(error) => error.to_string(),
    };

    println!("{}", response);
}
```
