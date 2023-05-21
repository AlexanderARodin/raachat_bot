#![allow(non_snake_case)]

use teloxide::prelude::*;
use std::sync::{Arc,Mutex};


#[tokio::main]
async fn main() {
    println!("initializing..");
    pretty_env_logger::init();
    log::info!("start testraa bot ...");

    let bot = Bot::from_env();
    let inoutSrc = Arc::new( Mutex::new( InOut::new() ) );
    let x = String::from("gtr");
    println!("started..");

    let inoutClone = inoutSrc.clone();
    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        let msg_text = msg.text();
        let chat_id = msg.chat.id;
        //let inout = inoutClone;
        let mvdStrng = x;
        match msg_text {
            None => println!("{} -> <empty>", chat_id),
            Some(text) => {
                println!("{} -> {}", chat_id, text);
                match text {
                    _ => {
                        println!("_=>");
                    }
                }
            },
        }
        Ok(())
    })
    .await;
    println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
}

struct InOut {
    senderId: Option<ChatId>,
    receiverId: Option<ChatId>,
}
impl InOut {
    fn new() -> InOut {
        InOut{ senderId:None, receiverId:None }
    }
}
