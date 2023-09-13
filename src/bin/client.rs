use async_std::io::BufReader;
use async_std::prelude::*;
use async_std::{task, io, net};
use std::sync::Arc;

use chat::utils::{self, ChatResult};
use chat::{Client, Server};

fn get_value(mut line: &str) -> Option<(&str, &str)> {
  line = line.trim_start();

  if line.is_empty() {
    return None;
  }

  match input.find(char::is_whitespace) {
    Some(idx) => Some((&line[0..idx], &line[idx..])),
    None => Some((line, "")),
  }
}

fn parse_input(line: &str) -> Option<Client> {
  let (input, remainder) = get_value(line)?;

  if input == "join"{
    let (chat, remainder) = get_value(remainder);
    
    
  }
}

async fn send(mut send: net::TcpStream) -> ChatResult<()>{
  println!("Options: \njoin CHAT\npost CHAT MESSAGE");

  let mut options = io::BufReader::new(io::stdin()).lines();

  while let Some(option_result) = options.next().await {
    let opt = option_result?;
    let req = match parse_input(&options){
      Some(req) => req,
      None => continue,
    };
    utils::send_json(&mut send, &req).await?;
    send.flush().await?;
  }
}


fn main(){

}