use std::{net::{TcpListener, TcpStream}, io::Read};
use std::io::Write;

fn handle_connection(mut stream: TcpStream) {
  let mut buffer = [0; 1028];
  let n = stream.read(&mut buffer[..]).unwrap();
  println!("Request: {}", String::from_utf8_lossy(&buffer[..n]));
  
  let response = "HTTP/1.1 200 OK\r\n\r\nHello World!";
  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();
}

fn main() {
// Start TCP Server that is listening on port 7878 which receives a question from IOT device as string
// Accept question and send it via HTTP request to LLM in format localhost:11434/api/generate
//'{
// "model": "llama2:latest",
// "prompt": ""
// }'
// 
// Receive the response from LLM as stream and send it to IOT device via TCP

  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

  for stream in listener.incoming() {
    let s: TcpStream = stream.unwrap();
    handle_connection(s);
    println!("Connection established");
  }
}
