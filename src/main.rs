use std::collections::HashMap;
use std::net::TcpListener;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::runtime::Runtime;

use reqwest::{Error, Client}; 

static LLM_URL: &str = "http://localhost:11434/api/generate";

// Async function to send the received question to the LLM
async fn send_received_question_to_llm(received_string: &str) -> Result<std::string::String, Error> {
    let mut req_body = HashMap::new();
    req_body.insert("model", "llama2");
    req_body.insert("prompt", received_string);

    let client = Client::new();
    match client.post(LLM_URL).json(&req_body).send().await {
        Ok(res) => {
            let body = res.text().await?;
            Ok(body)
        }        Err(err) => {
            println!("Request: Error {}", err);
            Err(err)
        }
    }
}

// Async function to handle each TCP connection
async fn handle_connection(mut stream: TcpStream) -> Result<(), Error> {
  let mut buffer = vec![0; 1024];

  let buf_len = stream.read(&mut buffer).await.unwrap();
  let sliced_buf = &buffer[..buf_len];

  let received_string = String::from_utf8_lossy(sliced_buf);
  println!("Received: {:?}", received_string);

  let res = send_received_question_to_llm(&received_string).await?;
  println!("Response status code: {}", res.to_string());

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
