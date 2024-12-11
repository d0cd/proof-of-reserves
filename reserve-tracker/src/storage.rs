use serde_json;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn save_addresses(addresses: &[String], file: &str) -> Result<(), std::io::Error> {
    let serialized = serde_json::to_string(&addresses)?;
    let mut f = tokio::fs::File::create(file).await?;
    f.write_all(serialized.as_bytes()).await?;
    Ok(())
}

pub async fn load_addresses(file: &str) -> Result<Vec<String>, std::io::Error> {
    match tokio::fs::File::open(file).await {
        Ok(mut f) => {
            let mut contents = String::new();
            f.read_to_string(&mut contents).await?;
            let addresses = serde_json::from_str(&contents)?;
            Ok(addresses)
        }
        Err(_) => Ok(Vec::new()),
    }
}

pub async fn save_transactions(transactions: &[String], file: &str) -> Result<(), std::io::Error> {
    let serialized = serde_json::to_string(transactions)?;
    let mut f = tokio::fs::File::create(file).await?;
    f.write_all(serialized.as_bytes()).await?;
    Ok(())
}

pub async fn load_transactions(file: &str) -> Result<Vec<String>, std::io::Error> {
    match tokio::fs::File::open(file).await {
        Ok(mut f) => {
            let mut contents = String::new();
            f.read_to_string(&mut contents).await?;
            let transactions = serde_json::from_str(&contents)?;
            Ok(transactions)
        }
        Err(_) => Ok(Vec::new()),
    }
}
