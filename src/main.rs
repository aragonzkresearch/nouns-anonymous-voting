use nouns_voting_client::run;

fn main() {
    // Collect all the arguments passed to the program
    // and pass them to the run function of the library.
    run().unwrap_or_else(|err| println!("Failed to run the client with error: {}", err));
}
