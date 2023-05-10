use std::env;
use std::process;

use telebot::Bot;
use telebot::functions::*;
use futures::stream::Stream;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("ERROR: there is no Token in command line");
        process::exit(1);
    }else{
        println!("start under token: {}", args[1] );
        start_bot( args[1].clone() );
    }
}

fn start_bot( bot_token: String ) {
    let mut bot = Bot::new( &bot_token );
    
    let handle = bot
        .new_cmd("/putin");

    bot.run_with( handle );
}
