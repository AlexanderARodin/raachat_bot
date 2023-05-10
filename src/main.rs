
use std::env;
use std::process;


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
    println!( "{}", bot_token );

}
