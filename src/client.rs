use crate::utils;
use crate::{Error, Result};
use borsh::BorshSerialize;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::Signer;

pub fn establish_connection() -> Result<RpcClient> {
    let rpc_url = utils::get_rpc_url()?;
    Ok(RpcClient::new_with_commitment(
        rpc_url,
        CommitmentConfig::confirmed(),
    ))
}

pub fn get_balance_requirement(connection: &RpcClient) -> Result<u64> {
    #[derive(BorshSerialize)]
    struct GreetingSchema {
        counter: u32,
    }
    let encoded = GreetingSchema { counter: 0 }
        .try_to_vec()
        .map_err(|e| Error::SerializationError(e))?;
    let account_fee = connection.get_minimum_balance_for_rent_exemption(encoded.len())?;

    let (_, fee_calculator) = connection.get_recent_blockhash()?;
    let transaction_fee = fee_calculator.lamports_per_signature * 100;

    Ok(transaction_fee + account_fee)
}

pub fn get_player_balance(
    player: &solana_sdk::signer::keypair::Keypair,
    connection: &RpcClient,
) -> Result<u64> {
    Ok(connection.get_balance(&player.pubkey())?)
}

pub fn request_airdrop(
    player: &solana_sdk::signer::keypair::Keypair,
    connection: &RpcClient,
    amount: u64,
) -> Result<()> {
    let balance = get_player_balance(player, connection)?;
    let sig = connection.request_airdrop(&player.pubkey(), amount)?;
    loop {
        let confirmed = connection.confirm_transaction(&sig)?;
        if confirmed {
            break;
        }
    }
    Ok(())
}
