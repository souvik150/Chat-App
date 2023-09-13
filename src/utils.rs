use async_std::prelude::*;

user serde::de::DeserializeOwned;
use serde::Serialize;
use std::error::Error;
use std::marker::Unpin;

pub type ChatError = Box<dyn Error + Send + Sync + 'static>;
pub type ChatResult<T> = Result<T, ChatError>;

pub async fn send_json<O,P>(leaving: &mut O, packetL &P) -> ChatResult<()>
where
    O: async_std::io::Write + Unpin,
    P: Serialize,
{
    let mut json = serde_json::to_string(&packet)?;
    json.push('\n');

    leaving.write_all(json.as_bytes()).await?;
    Ok(())
}

pub async fn recv_json<I, T>(incoming: I) -> impl Strem<Item = ChatResult<T>>
where
  I: async_std::io::BufRead + Unpin,
  T: DeserializeOwned,
{
  incoming
    .lines()
    .map(|line| ChatResult<T>{
      let line = line?;
      let packet = serde_json::from_str(&line)?;
      Ok(packet)
    }) 
}