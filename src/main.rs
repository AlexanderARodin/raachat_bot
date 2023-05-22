#![allow(non_snake_case)]

use teloxide::prelude::*;
use std::sync::Mutex;


static SENDER_MUT: Mutex<Option<ChatId>> = Mutex::new(None);
static RECEIVER_MUT: Mutex<Option<ChatId>> = Mutex::new(None);

#[tokio::main]
async fn main() {
    println!("initializing..");
    pretty_env_logger::init();
    log::info!("start testraa bot ...");

    let bot = Bot::from_env();
    println!("started..");

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        let msg_text = msg.text();
        let chat_id = msg.chat.id;
        match msg_text {
            None => println!("{} -> <empty>", chat_id),
            Some(text) => {
                println!("{} -> {}", chat_id, text);
                match text {
                    "/sender" => {
                        println!("[set SENDER]: {}", chat_id);
                        {
                            let mut senderId = SENDER_MUT.lock().unwrap();
                            *senderId = Some(chat_id);
                        }
                        bot.send_message(chat_id, "you are SENDER now")
                            .await?;
                    }
                    "/receiver" => {
                        println!("[set RECEIVER]: {}", chat_id);
                        {
                            let mut receiverId = RECEIVER_MUT.lock().unwrap();
                            *receiverId = Some(chat_id);
                        }
                        bot.send_message(chat_id, "you are RECEIVER now")
                            .await?;
                    }
                    _ => {
                        match getSenderReceiver() {
                            (None,None) => {
                                bot.send_message(chat_id, "there are no sender/receiver (yet).")
                                    .await?;
                            }
                            (None,_) => {
                                bot.send_message(chat_id, "there is no receiver (yet).")
                                    .await?;
                            }
                            (_,None) => {
                                bot.send_message(chat_id, "there is no sender (yet).")
                                    .await?;
                            }
                            (Some(sender), Some(receiver)) => {
                                if sender == chat_id {
                                    bot.send_message(receiver, text)
                                        .await?;
                                }else{
                                    bot.send_message(chat_id, "you are not sender (yet).")
                                        .await?;
                                }
                            }
                        }
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

fn getSenderReceiver() -> (Option<ChatId>,Option<ChatId>) {
    let senderId:Option<ChatId> = *SENDER_MUT.lock().unwrap();
    let receiverId:Option<ChatId> = *RECEIVER_MUT.lock().unwrap();

    (senderId,receiverId)
}
