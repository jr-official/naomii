mod ports;
mod server;

use std::io;
use server::setup;

#[tokio::main]
async fn main() {
    let mut cmd = String::new();
    io::stdin()
        .read_line(&mut cmd)
        .expect("Err reading line");

    match cmd.trim() {
        "!help" | "help" => {
            println!("\x1b[1;34mNaomiiStartup commands\x1b[0m"); 
            println!("\x1b[33m!help\x1b[0m, \x1b[33mhelp\x1b[0m - displays this menu");
            println!("\x1b[33m!startup\x1b[0m, \x1b[33mstart\x1b[0m - starts naomii installation if you dont already have it");
            println!("\x1b[33m!setnew\x1b[0m, \x1b[33mnew\x1b[0m, \x1b[33mset\x1b[0m - implements naomii router on a server");
        }
        "!setnew" | "set" | "new" => {
            setup().await;
        }
        _ => todo!()
    }
}

