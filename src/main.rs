
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    println!("initializing..");
    pretty_env_logger::init();
    log::info!("start testraa bot ...");

    let mut bot = Bot::from_env();
    
    println!("started..");
    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        println!("inside!!!: {:?}", msg.text());
        bot.send_message(msg.chat.id, "putin xuiylo").await?;
        Ok(())
    })
    .await;
}
