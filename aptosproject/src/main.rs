use aptos_sdk::transaction_builder::TransactionFactory;
use aptos_sdk::types::AccountAddress;
use aptos_sdk::client::Client;
use crate::auction::Auction; // Assuming your auction code is in `auction.rs`

fn main() -> Result<(), AptosError> {
    // Replace with your Aptos network node URL (e.g., devnet)
    let client = Client::new("https://fullnode.devnet.aptos.dev");

    // Example: Create an auction
    let account_address = AccountAddress::random(); // Replace with your account address
    let item = Auction::Item {
        name: "My Awesome Item".to_string(),
        description: "This is a great item you won't want to miss!".to_string(),
        image_url: "https://example.com/item.jpg".to_string(),
    };
    let start_time = 1663728000; // Example start time (timestamp in seconds)
    let end_time = 1663731600;   // Example end time (timestamp in seconds)
    let create_txn = Auction::create_auction(item, start_time, end_time, account_address, &client)?;
    client.submit_transaction(create_txn)?;

    // Example: Place a bid on an existing auction
    let auction_id: u64 = 123; // Replace with the actual auction ID
    let bidder = AccountAddress::random(); // Replace with your bidder address
    let amount = 1000; // Bid amount
    let place_bid_txn = Auction::place_bid(auction_id, bidder, amount, account_address, &client)?;
    client.submit_transaction(place_bid_txn)?;

    // Example: Get auction information
    let auction = Auction::get_auction(auction_id, &client)?;
    println!("Auction details: {:?}", auction);

    // Example: Get highest bid
    let highest_bid = Auction::get_highest_bid(auction_id, &client)?;
    println!("Highest bid: {:?}", highest_bid);

    // Example: Close auction
    let close_txn = Auction::close_auction(auction_id, account_address, &client)?;
    client.submit_transaction(close_txn)?;

    // Example: List all auctions
    let auctions = Auction::get_all_auctions(&client)?;
    println!("Active auctions: {:?}", auctions);

    // Example: Cancel auction (if allowed)
    let cancel_txn = Auction::cancel_auction(auction_id, account_address, &client)?;
    client.submit_transaction(cancel_txn)?;

    // Example: Extend auction duration (if allowed)
    let new_end_time = 1663735200; // New end time
    let extend_txn = Auction::extend_auction(auction_id, account_address, new_end_time, &client)?;
    client.submit_transaction(extend_txn)?;

    // ... (Add more auction interactions as needed)

    Ok(())
}