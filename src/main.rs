use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();

    log::info!("Starting repl_o_bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::repl(bot, |message| async move {
        match message.update.text() {
            Some("/start") => match message.update.from() {
                Some(user) => {
                    message
                        .answer(format!(
                            "Hi, {}!\n\nI can parse O language\n\nOne message = One Runtime",
                            user.username.as_ref().unwrap()
                        ))
                        .await
                        .log_on_error()
                        .await;
                }
                None => {
                    println!(" There is no User field in update ");
                }
            },

            Some(text) => {
                let url = Url::parse("wss://_o_WebSocket.link/ws").unwrap();

                let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
                // println!("WebSocket handshake has been successfully completed");

                let (mut write, read) = ws_stream.split();

                write
                    .send(Message::Text(text.to_string()))
                    .await
                    .expect("Failed to send message");

                read.for_each(|soc_message| async {
                    match soc_message {
                        Ok(answ) => {
                            message
                                .reply_to(answ.to_string())
                                .await
                                .log_on_error()
                                .await;
                        }
                        Err(e) => {
                            println!("{:?},{:?}", text, e);
                        }
                    }
                })
                .await;
            }

            None => {
                println!(" message does not have text ");
            }
        }
        respond(())
    })
    .await;
}
