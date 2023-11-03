use std::{error::Error, env, io::{stdout, Write, stdin}};

use dotenv::dotenv;
use hyper::{Client, Body, Request, header, body::Buf};
use hyper_tls::HttpsConnector;
use serde_derive::{Serialize, Deserialize};
use spinners::Spinner;


#[derive(Deserialize)]
struct OAIChoices {
    text: String,
    // index: u8,
    // logprobs: Option<u8>,
    // finish_reason: String
}


#[derive(Deserialize)]
struct OAIResponse {
    // id: Option<String>,
    // object: Option<String>,
    // created: Option<u64>,
    // model: Option<String>,
    choices: Vec<OAIChoices>,
}


#[derive(Serialize, Debug)]
struct OAIRequest {
    prompt: String,
    max_tokens: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    dotenv().ok();

    let http = HttpsConnector::new();
    let client = Client::builder()
                                                                .build::<_, hyper::Body>(
                                                                    http
                                                                );
    let uri = "https://api.openai.com/v1/completions";
    let preamble = "Generate a SQL code for the given statement";
    let oai_token : String = env::var("OAT_TOKEN").unwrap();

    let auth_header_val = format!("Bearer {}", oai_token);
    println!("{esc}c", esc = 27 as char);

    loop {
        print!(">");
        stdout().flush().unwrap();
        
        let mut user_text = String::new();

        stdin().read_line(&mut user_text).expect("failed to read line");

        println!("");

        let spinner = Spinner::new(
            &spinners::Spinners::Dots12, 
            "\t\tOpen Ai is thiinking ....".into()
        );

        let oai_request = OAIRequest {
            prompt: format!("{} {}", preamble, user_text),
            max_tokens: 1000,
        };

        let body = Body::from(serde_json::to_vec(&oai_request)?);

        let req = Request::post(uri)
        .header(header::CONTENT_TYPE, "application/json")
        .header("Authorization", &auth_header_val)
        .body(body)
        .unwrap();

        let res = client.request(req).await?;
        let body = hyper::body::aggregate(res).await?;
        let json : OAIResponse = serde_json::from_reader(body.reader())?;
        spinner.stop();
        println!("");
        println!("{}", json.choices[0].text);
    }
}
