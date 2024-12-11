use snarkvm::prelude::{Result, Network, Transaction, anyhow, ConfirmedTransaction};

/// A utility to broadcast a transaction.
pub fn broadcast_transaction<N: Network>(
    transaction: &Transaction<N>,
    endpoint: &str,
    network: &str,
) -> Result<N::TransactionID> {
    // Construct a URL to broadcast a transaction.
    let url = format!("{}/{}/transaction/broadcast", endpoint, network);
    // Broadcast the transaction.
    // The transaction should be JSON.
    let response = ureq::post(&url)
        .send_json(serde_json::to_value(transaction)?)
        .map_err(|e| anyhow!("Failed to broadcast transaction: {}", e))?;
    // Get the transaction ID from the response.
    let tx_id = response.into_json::<N::TransactionID>()
        .map_err(|e| anyhow!("Failed to get transaction ID: {}", e))?;

    Ok(tx_id)
}

/// A utility to get the block height from a transaction.
pub fn get_block_height<N: Network>(
    tx_id: &str,
    endpoint: &str,
    network: &str,
) -> Result<String> {
    // Construct a URL to get the block hash from a transaction.
    let url = format!("{}/{}/find/blockHash/{}", endpoint, network, tx_id);
    // Get the block hash from the transaction.
    // The response should be JSON with a field `blockHash` that denotes the block hash.
    let response = ureq::get(&url)
        .call()
        .map_err(|e| anyhow!("Failed to get block hash: {}", e))?;
    // Get the block hash from the response.
    // Remove " from the block hash.
    let block_hash = response.into_json::<serde_json::Value>()
        .map_err(|e| anyhow!("Failed to get block hash: {}", e))?.to_string().replace("\"", "");

    // Construct a URL to get the block height from the block hash.
    let url = format!("{}/{}/height/{}", endpoint, network, block_hash);
    // Get the block height from the block hash.
    // The response should be JSON with a field `height` that denotes the block height.
    let response = ureq::get(&url)
        .call()
        .map_err(|e| anyhow!("Failed to get block height: {}", e))?;
    // Get the block height from the response.
    let height = response.into_json::<serde_json::Value>()
        .map_err(|e| anyhow!("Failed to get block height: {}", e))?.to_string().replace("\"", "");

    Ok(height)
}

/// A utility to get the block given a height.
pub fn get_block_timestamp<N: Network>(
    height: &str,
    endpoint: &str,
    network: &str,
) -> Result<String> {
    // Construct a URL to get the block given a height.
    let url = format!("{}/{}/block/{}", endpoint, network, height);
    // Get the block given a height.
    // The response should be JSON with the block object.
    let response = ureq::get(&url)
        .call()
        .map_err(|e| anyhow!("Failed to get block: {}", e))?;
    // Get the block object from the response.
    let block = response.into_json::<serde_json::Value>()
        .map_err(|e| anyhow!("Failed to get block: {}", e))?;
    // Get the timestamp.
    let timestamp = block.get("header").unwrap().get("metadata").unwrap().get("timestamp").unwrap().to_string().replace("\"", "");
    let timestamp = timestamp.parse::<i64>().unwrap();
    // Pretty print the timestamp.
    let block = chrono::DateTime::from_timestamp(timestamp as i64, 0).unwrap().to_string();

    Ok(block)
}

/// A utility to get the confirmed transaction object.
pub fn get_confirmed_transaction<N: Network>(
    tx_id: &str,
    endpoint: &str,
    network: &str,
) -> Result<ConfirmedTransaction<N>> {
    // Construct a URL to get the confirmed transaction object.
    let url = format!("{}/{}/transaction/confirmed/{}", endpoint, network, tx_id);
    // Get the confirmed transaction object.
    // The response should be JSON with the confirmed transaction object.
    let response = ureq::get(&url)
        .call()
        .map_err(|e| anyhow!("Failed to get confirmed transaction: {}", e))?;
    // Get the confirmed transaction object from the response.
    let tx = response.into_json::<ConfirmedTransaction<N>>()
        .map_err(|e| anyhow!("Failed to get confirmed transaction: {}", e))?;

    Ok(tx)
}

/// A utility to query a mapping value.
pub fn get_mapping_value<N: Network>(
    program_id: &str,
    mapping_name: &str,
    key: &str,
    endpoint: &str,
    network: &str,
) -> Result<String> {
    // Construct a URL to query a mapping value.
    let url = format!("{}/{}/program/{program_id}/mapping/{mapping_name}/{key}", endpoint, network);
    println!("Querying mapping value: {}", url);
    // Query the mapping value.
    // The response should be JSON with the mapping value.
    let response = ureq::get(&url)
        .call()
        .map_err(|e| anyhow!("Failed to query mapping value: {}", e))?;
    // Get the mapping value from the response.
    let value = response.into_json::<serde_json::Value>()
        .map_err(|e| anyhow!("Failed to query mapping value: {}", e))?.as_str().unwrap().to_string();

    Ok(value)
}
