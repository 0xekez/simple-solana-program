use zeke_contract as zc;

fn main() {
    let connection = zc::client::establish_connection().unwrap();
    println!(
        "Connected to remote solana node running version ({}).",
        connection.get_version().unwrap()
    );

    let balance_requirement = zc::client::get_balance_requirement(&connection).unwrap();
    println!(
        "({}) lamports are required for this transaction.",
        balance_requirement
    );

    let player = zc::utils::get_payer().unwrap();
    let player_balance = zc::client::get_player_balance(&player, &connection).unwrap();
    println!("({}) lamports are owned by player.", player_balance);

    if player_balance < balance_requirement {
        let request = balance_requirement - player_balance;
        println!(
            "Player does not own sufficent lamports. Airdropping ({}) lamports.",
            request
        );
        zc::client::request_airdrop(&player, &connection, request).unwrap();
    }

    // TODO:
    //   1. Check that the progra exists
    //   2. Send a transaction to the program
    //   3. Query the program's account for the result of the transaction.
}
