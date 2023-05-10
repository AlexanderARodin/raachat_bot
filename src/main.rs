use std::env;
use std::process;

use tbot::prelude::*;

#[tokio::main]
async fn main() {
    println!("initializing..");
    let mut bot = tbot::from_env!( "BOT_TOKEN" ).event_loop();
    
    bot.text( |context| async move {
        println!("inside!!");
    });

    println!("started..");
    bot.polling().start().await.unwrap();
}
