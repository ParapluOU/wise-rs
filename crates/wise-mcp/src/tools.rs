//! MCP tool implementations.

use wise_client::ReadOnlyClient;

/// Get the authenticated user's information.
pub async fn get_user(client: &ReadOnlyClient) -> Result<String, wise_client::error::Error> {
    let user = client.user().get().await?;
    Ok(serde_json::to_string_pretty(&user).unwrap_or_else(|_| format!("{:?}", user)))
}

/// List all profiles for the user.
pub async fn list_profiles(client: &ReadOnlyClient) -> Result<String, wise_client::error::Error> {
    let profiles = client.profiles().list().await?;
    Ok(serde_json::to_string_pretty(&profiles).unwrap_or_else(|_| format!("{:?}", profiles)))
}

/// List balances for a profile.
pub async fn list_balances(
    client: &ReadOnlyClient,
    profile_id: i64,
) -> Result<String, wise_client::error::Error> {
    let balances = client.balances().list(profile_id).await?;
    Ok(serde_json::to_string_pretty(&balances).unwrap_or_else(|_| format!("{:?}", balances)))
}

/// List transfers.
pub async fn list_transfers(
    client: &ReadOnlyClient,
    profile_id: Option<i64>,
    limit: Option<u32>,
) -> Result<String, wise_client::error::Error> {
    let transfers = client.transfers().list(profile_id, limit, None).await?;
    Ok(serde_json::to_string_pretty(&transfers).unwrap_or_else(|_| format!("{:?}", transfers)))
}

/// Get a specific transfer.
pub async fn get_transfer(
    client: &ReadOnlyClient,
    transfer_id: i64,
) -> Result<String, wise_client::error::Error> {
    let transfer = client.transfers().get(transfer_id).await?;
    Ok(serde_json::to_string_pretty(&transfer).unwrap_or_else(|_| format!("{:?}", transfer)))
}

/// Get exchange rate.
pub async fn get_rate(
    client: &ReadOnlyClient,
    source: &str,
    target: &str,
) -> Result<String, wise_client::error::Error> {
    let rate = client.rates().get(source, target).await?;
    Ok(serde_json::to_string_pretty(&rate).unwrap_or_else(|_| format!("{:?}", rate)))
}

/// List recipient accounts.
pub async fn list_recipients(
    client: &ReadOnlyClient,
    profile_id: i64,
) -> Result<String, wise_client::error::Error> {
    let recipients = client.recipients().list(Some(profile_id), None).await?;
    Ok(serde_json::to_string_pretty(&recipients).unwrap_or_else(|_| format!("{:?}", recipients)))
}
