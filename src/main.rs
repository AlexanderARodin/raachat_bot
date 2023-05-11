use teloxide::prelude::*;


static mut senderID: ChatId = teloxide::prelude::ChatId(0);
static mut receiverID: ChatId = teloxide::prelude::ChatId(0);
 

#[tokio::main]
async fn main() {
    println!("initializing..");
    pretty_env_logger::init();
    log::info!("start testraa bot ...");

    let mut bot = Bot::from_env();
    
    println!("started..");
    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        let msg_text = msg.text();
        let chat_id = msg.chat.id;
        match msg_text {
            None => println!("{} -> <empty>", chat_id),
            Some(text) => {
                println!("{} -> {}", chat_id, text);
              unsafe{
                match text {
                    "/sender" => {
                        senderID = chat_id;
                        bot.send_message(chat_id, "you are sender").await?;
                    },
                    "/receiver" => {
                        receiverID = chat_id;
                        bot.send_message(chat_id, "you are receiver").await?;
                    },
                    _ => {
                        if receiverID == teloxide::prelude::ChatId(0){
                            bot.send_message(chat_id, "there is not receiver").await?;
                        }else{
                            bot.send_message(receiverID, text).await?;
                        }
                    },
                }
              }
            },
        }
        Ok(())
    })
    .await;
}



