use teloxide::prelude::*;
use std::sync::{Arc,Mutex};


static senderID_Mx:   Arc<Mutex<ChatId>> = Arc::new( Mutex::new(ChatId(0)) );
static receiverID_Mx: Arc<Mutex<ChatId>> = Arc::new( Mutex::new(ChatId(0)) );

#[tokio::main]
async fn main() {
    
    println!("initializing..");
    pretty_env_logger::init();
    log::info!("start testraa bot ...");

    let bot = Bot::from_env();
    
    println!("started..");
    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        replyer( bot, msg );
        Ok(())
    })
    .await;
}

async fn replyer( bot: Bot, msg: Message ) {
        let msg_text = msg.text();
        let chat_id = msg.chat.id;
        match msg_text {
            None => println!("{} -> <empty>", chat_id),
            Some(text) => {
                println!("{} -> {}", chat_id, text);
              match text {
                  "/sender" => {
                      println!("sender =>");
                      let mut senderID = senderID_Mx.lock().unwrap();
                      //*senderID = chat_id;
                      //bot.send_message(chat_id, "you are sender").await?;
                  },
                  "/receiver" => {
                      println!("receiver =>");
                      //let mut receiverID = receiverID_Mx.lock().unwrap();
                      //*receiverID = chat_id;
                      //bot.send_message(chat_id, "you are receiver").await?;
                  },
                  _ => {
                      println!("_ =>");
                      //let receiverID = receiverID_Mx.lock().unwrap();
                      //if receiverID.0 == 0 {
                      //    bot.send_message(chat_id, "there is not receiver").await?;
                      //}else{
                      //    bot.send_message( ChatId(0), text ).await?;
                      //}
                  },
              }
            },
        }
}


