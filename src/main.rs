use zeke_contract as zc;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!(
            "usage: {} <path to solana hello world example program keypair>",
            args[0]
        );
        std::process::exit(-1);
    }
    let keypair_path = &args[1];

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

    let player = zc::utils::get_player().unwrap();
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

    let program = zc::client::get_program(keypair_path, &connection).unwrap();

    zc::client::create_greeting_account(&player, &program, &connection).unwrap();

    zc::client::say_hello(&player, &program, &connection).unwrap();
    println!(
        "({}) greetings have been sent.",
        zc::client::count_greetings(&player, &program, &connection).unwrap()
    )
}
