use discord_rpc_client::Client;

fn main() {
    const APP_ID: u64 = 1443580874888188007;

    let mut client = Client::new(APP_ID);

    client.start();

    client
        .set_activity(|act| {
            act.details("Test performative app")
                .state("Book: Laut Bercerita")
        })
        .expect("Failed to set activity");

    println!("Discord presence active");

    loop {}
}
