use discord_rpc_client::Client;
use std::io::{self, BufRead, Write};

use crate::fetch::fetch_book;

mod fetch;

fn main() {
    let stdin = io::stdin();
    const APP_ID: u64 = 1443580874888188007;
    let mut client = Client::new(APP_ID);

    client.start();

    println!("===================================");
    println!("WELCOME! Let's become performative!");
    println!("CTRL + C to stop");
    println!("===================================");
    print!("Enter Book Name > ");
    io::stdout().flush().unwrap();

    for line in stdin.lock().lines() {
        // Take user's input

        // Fetch Google Books API
        let query = line.unwrap();

        match fetch_book(&query) {
            Ok((title, author)) => {
                println!("Found: {} by {}", title, author);

                // Set Client
                client
                    .set_activity(|act| {
                        act.details(format!("Reading {}", title))
                            .state(format!("By {}", author))
                    })
                    .expect("Failed to set activity");

                println!("Status updated");
            }
            Err(error) => println!("Error: {}", error),
        }

        println!("===================================");
        println!("Discord presence active!");
        println!("CTRL + C to stop");
        print!("Reset Book Name > ");
        io::stdout().flush().unwrap();
    }
}
