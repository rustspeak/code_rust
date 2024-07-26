use aptos_sdk::{
    client::RestClient,
    types::{account_address::AccountAddress, transaction::SignedTransaction},
    crypto::ed25519::Ed25519PrivateKey,
};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::str::FromStr;
use rand::rngs::OsRng;
use aptos_sdk::types::LocalAccount;
use aptos_sdk::crypto::PrivateKey;

#[derive(Serialize, Deserialize)]
struct Project {
    id: u64,
    name: String,
    description: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let rest_client = RestClient::new("https://fullnode.devnet.aptoslabs.com".to_string());

    // Créer un nouveau compte
    let new_account = create_account(&rest_client).await?;
    println!("New account created: {:?}", new_account);

    // Exemple de projet à transformer en NFT
    let project = Project {
        id: 1,
        name: "Projet Test".to_string(),
        description: "Description du projet".to_string(),
    };

    // Appeler la fonction de transformation du projet en NFT
    transform_project_to_nft(&rest_client, &new_account, project).await?;

    Ok(())
}

async fn create_account(client: &RestClient) -> Result<LocalAccount> {
    // Générer une clé privée pour le nouveau compte
    let mut rng = OsRng;
    let private_key = Ed25519PrivateKey::generate(&mut rng);
    let account = LocalAccount::new(AccountAddress::random(), private_key, 0);

    // Soumettre une transaction pour créer un compte sur la blockchain Aptos
    let txn_payload = serde_json::json!({
        "type": "entry_function_payload",
        "function": "0x1::aptos_account::create_account",
        "arguments": [account.address().to_string()],
        "type_arguments": []
    });

    let signed_txn = SignedTransaction::new(
        account.address(),
        txn_payload,
        0, // sequence number
        1000, // max gas amount
        1, // gas unit price
        "0x1::aptos_coin::AptosCoin".to_string(),
    );

    client.submit_transaction(&signed_txn).await?;

    Ok(account)
}

async fn transform_project_to_nft(client: &RestClient, account: &LocalAccount, project: Project) -> Result<()> {
    println!("Transforming project {} into NFT...", project.name);

    // Exemple de transaction simple (à personnaliser selon vos besoins)
    let txn_payload = serde_json::json!({
        "type": "entry_function_payload",
        "function": "0x1::example_nft::create_nft",
        "arguments": [
            project.id.to_string(),
            project.name,
            project.description
        ],
        "type_arguments": []
    });

    // Crée une transaction signée
    let signed_txn = SignedTransaction::new(
        account.address(),
        txn_payload,
        account.sequence_number(), // obtenir le numéro de séquence du compte
        1000, // max gas amount
        1, // gas unit price
        "0x1::aptos_coin::AptosCoin".to_string(), // currency code
    );

    match client.submit_transaction(&signed_txn).await {
        Ok(response) => println!("Transaction submitted: {:?}", response),
        Err(e) => eprintln!("Error submitting transaction: {:?}", e),
    }

    Ok(())
}

