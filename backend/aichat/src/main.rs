use std::fs;
use std::process;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::{json, Value};
use serde::{Deserialize, Serialize};
use std::io::{self, Read};
use structopt::StructOpt;
use std::collections::HashMap;

//.game__about-text > div > p
// "https://rawg.io/games/mirrors-edge"

#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(help = "Path to the API key file")]
    config_file: String,

    #[structopt(help = "Message to send to the API")]
    msg: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    api_key: String,
    model: String
}

#[derive(Debug, Deserialize, Serialize)]
struct GameInfo {
    game_title: String,
    rawg_url: String
}

fn read_file(api_key_file: &str) -> Result<String, io::Error> {
    let mut file = fs::File::open(api_key_file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.trim().to_owned())
}

fn gen_rawg_url(game_title: &str) -> String {
    let filtered_chars: String = game_title
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect();

    let processed: String = filtered_chars
        .split_whitespace()
        .map(|s| s.to_lowercase())
        .collect::<Vec<String>>()
        .join("-");

    let url = "https://rawg.io/games/".to_string() + &processed;

    url
}


// TODO: add side-effect that populates this with more metadata.
fn get_game_info(game_title: &str)-> GameInfo {
    GameInfo {
        rawg_url: gen_rawg_url(game_title),
        game_title: game_title.to_string()
    }
}



fn get_object<'a>(json_data: &'a Value, path: &[&str]) -> Option<&'a Value> {
    let mut current_value = json_data;

    for segment in path {
        if let Some(index) = parse_array_index(segment) {
            if let Some(value) = current_value.as_array()?.get(index) {
                current_value = value;
            } else {
                return None;
            }
        } else {
            if let Some(value) = current_value.get(segment) {
                current_value = value;
            } else {
                return None;
            }
        }
    }

    Some(current_value)
}

fn parse_array_index(segment: &str) -> Option<usize> {
    segment.parse().ok()
}

/*async fn ask_gpt(client: reqwest::Client, body: serde_json::Value) -> Result<serde_json::Value> {
    let api_url = "https://api.openai.com/v1/chat/completions";
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(AUTHORIZATION, format!("Bearer {}", config.api_key).parse()?);
    let response = client
        .post(api_url)
        .headers(headers)
        .json(&body)
        .send()
        .await?;

    let response_body = response.text().await?;
    let json_data: serde_json::Value = serde_json::from_str(&response_body)?;

    Ok(json_data)
}*/


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::from_args();
    let message = &args.msg;
    let config_file = &args.config_file;

    if message.is_empty() {
        println!("Needs a message as an argument, exiting.");
        process::exit(1);
    }

    if config_file.is_empty() {
        println!("Need an API key filepath but none was provided, exiting.");
        process::exit(1);
    }

    let config_str = read_file(config_file)?;
    let config: Config = serde_json::from_str(&config_str)?;

    let body = json!({
        "model": config.model,
        "messages": [
            {
                "role": "user",
                "content": message
            }
        ],
        "functions": [ {
            "name":"get_game_info",
            "description": "get information about the game passed as a parameter",
            "parameters": {
                "type": "object",
                "properties": {
                    "game_title": {
                        "type": "string",
                        "description": "The title of the game being asked about"
                    },
                    "rawg_url": {
                        "type":"string",
                        "description": "Url for additional metadata for the game being asked about."
                    }
                },
                "required": ["game_title"]
            }
        } ],
        "function_call": "auto"
    });

    let client = reqwest::Client::new();
    let api_url = "https://api.openai.com/v1/chat/completions";
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(AUTHORIZATION, format!("Bearer {}", config.api_key).parse()?);
    let response = client
        .post(api_url)
        .headers(headers)
        .json(&body)
        .send()
        .await?;

    let response_body = response.text().await?;
    let json_data: serde_json::Value = serde_json::from_str(&response_body)?;

    let function_call_path = &["choices", "0", "message", "function_call"];

    if let Some(function_call) = get_object(&json_data, function_call_path) {
        if let Some(fn_args_string) = function_call["arguments"].as_str() {
            let function_args: serde_json::Value = serde_json::from_str(fn_args_string)?;

            let mut available_functions: HashMap<String, Box<dyn Fn(&str)-> GameInfo>> = HashMap::new();
            available_functions.insert("get_game_info".to_string(), Box::new(|title| get_game_info(title)));

            if let Some(function_name) = function_call["name"].as_str() {
                if let Some(f) = available_functions.get(function_name) {
                    if let Some(game_title) = function_args["game_title"].as_str() {
                        let game_info: String = serde_json::to_string(&json!(f(game_title))).unwrap();
                        let new_message = json!({
                            "role": "function",
                            "name": function_name,
                            "content": game_info
                        });

                        if let Some(old_messages) = body["messages"].as_array() {
                            // println!("old_messages {:?}", old_messages);
                            let mut new_messages = old_messages.clone();
                            new_messages.push(new_message);
                            let new_messages_json = json!(new_messages);

                            if let Some(old_body) = body.as_object() {
                                let mut new_body = old_body.clone();
                                new_body.insert("messages".to_string(), new_messages_json);
                                let new_body_json = json!(new_body);

                                let mut headers = HeaderMap::new();
                                headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
                                headers.insert(AUTHORIZATION, format!("Bearer {}", config.api_key).parse()?);
                                let response = client
                                    .post(api_url)
                                    .headers(headers)
                                    .json(&new_body_json)
                                    .send()
                                    .await?;

                                let response_body = response.text().await?;
                                let json_data: serde_json::Value = serde_json::from_str(&response_body)?;
                                if let Some(reply) = json_data["choices"][0]["message"]["content"].as_str() {
                                    println!("{}", reply)
                                } else {
                                    println!("{}", json!({"error": "API issue, unable to retrieve data."}))
                                }
                            }
                        }

                    }

                }
            }
        }

    } else {
        if let Some(reply) = json_data["choices"][0]["message"]["content"].as_str() {
            println!("{}", reply);
        } else {
            println!("{}", json!({"error": "API issue, unable to retrieve data."}));
            process::exit(1);
        }

    }

    Ok(())
}
