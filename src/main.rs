use std::slice::SliceIndex;
use std::{collections::HashMap, vec};
use std::error::Error;
use std::net::TcpListener;
use serde_json::Value;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::runtime::Runtime;
use avtp_protocol::{send_data, receive_data};
use reqwest::Client; 
use base64::{Engine as _, alphabet, engine::{self, general_purpose}};

static LLM_URL: &str = "http://localhost:11434/api/generate";
static TTS_URL: &str = "http://localhost:5000/api/audio/tts";


fn base64_to_bytes(base64_str: &str) -> Vec<u8> {
    if base64_str.is_empty() {
        panic!("Response string was empty, could not convert to bytes")
    }

    match general_purpose::STANDARD.decode(base64_str) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Error decoding base64: {}", e);
            Vec::new()
        }
    }
}

async fn convert_response_to_mp3_base64(response_str: &str) -> Result<String, reqwest::Error> {
    let mut req_body = HashMap::new();
    req_body.insert("message", response_str);

    let client = Client::new();
    
    match client
    .post(TTS_URL)
    .json(&req_body).send()
    .await {
        Ok(res) => {
            let body = res.text().await.unwrap();
            Ok(body)
        }   
        Err(err) => {
            println!("Request: Error {}", err);
            Err(err)
        }
    }
}

fn transform_vec_jsonstrings_to_jsonobj(jsonarray_responses: Vec<String>) -> Vec<serde_json::Value> {
    jsonarray_responses.into_iter().map(|s| {
        serde_json::from_str(&s).unwrap_or_else(|_| serde_json::Value::Null)
    }).collect() 
}

fn create_full_response_string(target_jsonarray: &Vec<Value>) -> String {
    let mut full_response_string = String::new();

    for json in target_jsonarray.iter() {
       if json.is_object() {
            let response_str = json.get("response")
                .and_then(serde_json::Value::as_str)
                .unwrap_or_default();

            let is_done = json.get("done")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(true);

            println!("Part of response: {}", response_str);
            
            if !is_done {
               full_response_string.push_str(response_str); 
            }
        } else {
            println!("JSON is null or not an object");
        }   
    }

    full_response_string.trim().to_string()
}

fn propagate_response_jsonarray(llm_response_str: String, jsonarray_responses: &mut Vec<String>) -> Result<(), Box<dyn Error>> {
    let found_occurrences_closing_json: Vec<_> = llm_response_str.match_indices("}").collect();
    
    if found_occurrences_closing_json.is_empty() {
        return Err("No closing JSON brace found".into());
    }

    let mut i = 0;

    for (index, _) in found_occurrences_closing_json {
        let substring = llm_response_str[i..=index].to_string();

        let formatted_substring = substring.replace("\\n\\n", "").replace("\\", "");
        jsonarray_responses.push(formatted_substring);

        i = index + 1;
    }

    Ok(())
}

// Async function to send the received question to the LLM
async fn send_received_question_to_llm(received_string: &str) -> Result<String, reqwest::Error> {
    let mut req_body = HashMap::new();
    req_body.insert("model", "llama2");
    req_body.insert("prompt", received_string);

    let client = Client::new();
    match client.post(LLM_URL).json(&req_body).send().await {
        Ok(res) => {
            let body = res.text().await?;
            Ok(body)
        }   
        Err(err) => {
            println!("Request: Error {}", err);
            Err(err)
        }
    }
}

// Async function to handle each TCP connection
async fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = vec![0; 1024];
    let mut jsonarray_responses: Vec<String> = Vec::new();


    let buf_len = stream.read(&mut buffer).await.unwrap();
    let sliced_buf = &buffer[..buf_len];

    let received_string = String::from_utf8_lossy(sliced_buf);
    println!("Received: {:?}", received_string);

    let res = send_received_question_to_llm(&received_string).await?;
    println!("Got response now propagating the response jsonarray...");
    //println!("Response status code: {}", res);

    propagate_response_jsonarray(res, jsonarray_responses.as_mut())?;
    
    let target_jsonarray = transform_vec_jsonstrings_to_jsonobj(jsonarray_responses);

    let full_res_string = create_full_response_string(&target_jsonarray);

    println!("The full response is: {}", full_res_string);

    let base64_res_str = convert_response_to_mp3_base64(full_res_string.as_str())
        .await.unwrap_or(String::new());

    print!("Base64 Result: {}", base64_res_str);

    let byte_arr = base64_to_bytes(base64_res_str.as_str());
    let byte_slice = byte_arr.as_slice();

    send_data(stream.into_std().unwrap(), &byte_slice, "binary")?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let rt = Runtime::new()?;
  rt.block_on(async {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Started TCP listener on port 7878");
   let listener = tokio::net::TcpListener::from_std(listener).unwrap();

    loop {
      let (stream, _) = listener.accept().await.unwrap();
      println!("Connection established");

      // Spawning a new async task for each connection
      tokio::spawn(async move {
        if let Err(e) = handle_connection(stream).await {
            println!("Failed to handle connection: {}", e);
        }
      });
    }
  });

  Ok(())
}
