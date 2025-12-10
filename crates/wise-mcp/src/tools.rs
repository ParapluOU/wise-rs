//! MCP tool implementations.

use chrono::{DateTime, Utc};
use wise_client::models::{DirectDebitType, DisputeStatus, StatementType};
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
    let balances = client.balances().list(profile_id, None).await?;
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

/// List activities for a profile.
pub async fn list_activities(
    client: &ReadOnlyClient,
    profile_id: i64,
    size: Option<u32>,
) -> Result<String, wise_client::error::Error> {
    let mut params = wise_client::ListActivitiesParams::default();
    params.size = size;
    let activities = client.activities().list(profile_id, Some(params)).await?;
    Ok(serde_json::to_string_pretty(&activities).unwrap_or_else(|_| format!("{:?}", activities)))
}

/// Get an address by ID.
pub async fn get_address(
    client: &ReadOnlyClient,
    address_id: i64,
) -> Result<String, wise_client::error::Error> {
    let address = client.addresses().get(address_id).await?;
    Ok(serde_json::to_string_pretty(&address).unwrap_or_else(|_| format!("{:?}", address)))
}

/// List addresses for a profile.
pub async fn list_addresses(
    client: &ReadOnlyClient,
    profile_id: i64,
) -> Result<String, wise_client::error::Error> {
    let addresses = client.addresses().list(profile_id).await?;
    Ok(serde_json::to_string_pretty(&addresses).unwrap_or_else(|_| format!("{:?}", addresses)))
}

/// Get a balance statement.
pub async fn get_statement(
    client: &ReadOnlyClient,
    profile_id: i64,
    balance_id: i64,
    currency: &str,
    interval_start: DateTime<Utc>,
    interval_end: DateTime<Utc>,
) -> Result<String, wise_client::error::Error> {
    let statement = client
        .statements()
        .get_json(
            profile_id,
            balance_id,
            currency,
            interval_start,
            interval_end,
            StatementType::Compact,
            None,
        )
        .await?;
    Ok(serde_json::to_string_pretty(&statement).unwrap_or_else(|_| format!("{:?}", statement)))
}

/// List bank account details for a profile.
pub async fn list_bank_details(
    client: &ReadOnlyClient,
    profile_id: i64,
) -> Result<String, wise_client::error::Error> {
    let details = client.bank_details().list(profile_id).await?;
    Ok(serde_json::to_string_pretty(&details).unwrap_or_else(|_| format!("{:?}", details)))
}

/// Get a batch group by ID.
pub async fn get_batch_group(
    client: &ReadOnlyClient,
    profile_id: i64,
    batch_group_id: &str,
) -> Result<String, wise_client::error::Error> {
    let uuid = uuid::Uuid::parse_str(batch_group_id)
        .map_err(|e| wise_client::error::Error::Config(format!("Invalid UUID: {}", e)))?;
    let batch = client.batch_groups().get(profile_id, uuid).await?;
    Ok(serde_json::to_string_pretty(&batch).unwrap_or_else(|_| format!("{:?}", batch)))
}

/// List cards for a profile.
pub async fn list_cards(
    client: &ReadOnlyClient,
    profile_id: i64,
    page_size: Option<u32>,
    page_number: Option<u32>,
) -> Result<String, wise_client::error::Error> {
    let cards = client.cards().list(profile_id, page_size, page_number).await?;
    Ok(serde_json::to_string_pretty(&cards).unwrap_or_else(|_| format!("{:?}", cards)))
}

/// Get a card by token.
pub async fn get_card(
    client: &ReadOnlyClient,
    profile_id: i64,
    card_token: &str,
) -> Result<String, wise_client::error::Error> {
    let card = client.cards().get(profile_id, card_token).await?;
    Ok(serde_json::to_string_pretty(&card).unwrap_or_else(|_| format!("{:?}", card)))
}

/// List card orders for a profile.
pub async fn list_card_orders(
    client: &ReadOnlyClient,
    profile_id: i64,
    page_size: Option<u32>,
    page_number: Option<u32>,
) -> Result<String, wise_client::error::Error> {
    let orders = client
        .card_orders()
        .list(profile_id, page_size, page_number)
        .await?;
    Ok(serde_json::to_string_pretty(&orders).unwrap_or_else(|_| format!("{:?}", orders)))
}

/// Get a card order by ID.
pub async fn get_card_order(
    client: &ReadOnlyClient,
    profile_id: i64,
    card_order_id: i64,
) -> Result<String, wise_client::error::Error> {
    let order = client.card_orders().get(profile_id, card_order_id).await?;
    Ok(serde_json::to_string_pretty(&order).unwrap_or_else(|_| format!("{:?}", order)))
}

/// Get a card transaction by ID.
pub async fn get_card_transaction(
    client: &ReadOnlyClient,
    profile_id: i64,
    transaction_id: &str,
) -> Result<String, wise_client::error::Error> {
    let tx = client
        .card_transactions()
        .get(profile_id, transaction_id)
        .await?;
    Ok(serde_json::to_string_pretty(&tx).unwrap_or_else(|_| format!("{:?}", tx)))
}

/// Get a partner case by ID.
pub async fn get_case(
    client: &ReadOnlyClient,
    case_id: i64,
) -> Result<String, wise_client::error::Error> {
    let case = client.cases().get(case_id).await?;
    Ok(serde_json::to_string_pretty(&case).unwrap_or_else(|_| format!("{:?}", case)))
}

/// Get comments for a partner case.
pub async fn get_case_comments(
    client: &ReadOnlyClient,
    case_id: i64,
) -> Result<String, wise_client::error::Error> {
    let comments = client.cases().get_comments(case_id).await?;
    Ok(serde_json::to_string_pretty(&comments).unwrap_or_else(|_| format!("{:?}", comments)))
}

/// List direct debit accounts for a profile.
pub async fn list_direct_debits(
    client: &ReadOnlyClient,
    profile_id: i64,
    debit_type: &str,
    currency: &str,
) -> Result<String, wise_client::error::Error> {
    let dt = match debit_type.to_uppercase().as_str() {
        "ACH" => DirectDebitType::Ach,
        "EFT" => DirectDebitType::Eft,
        _ => {
            return Err(wise_client::error::Error::Config(
                "debit_type must be ACH or EFT".to_string(),
            ))
        }
    };
    let accounts = client.direct_debits().list(profile_id, dt, currency).await?;
    Ok(serde_json::to_string_pretty(&accounts).unwrap_or_else(|_| format!("{:?}", accounts)))
}

/// List disputes for a profile.
pub async fn list_disputes(
    client: &ReadOnlyClient,
    profile_id: i64,
    status: Option<&str>,
    page_size: Option<u32>,
    page_number: Option<u32>,
) -> Result<String, wise_client::error::Error> {
    let status_enum = match status {
        Some("ACTIVE") => Some(DisputeStatus::Active),
        Some("CLOSED") => Some(DisputeStatus::Closed),
        Some(s) => {
            return Err(wise_client::error::Error::Config(format!(
                "Invalid status: {}. Must be ACTIVE or CLOSED",
                s
            )))
        }
        None => None,
    };
    let disputes = client
        .disputes()
        .list(profile_id, status_enum, None, page_size, page_number)
        .await?;
    Ok(serde_json::to_string_pretty(&disputes).unwrap_or_else(|_| format!("{:?}", disputes)))
}

/// Get a dispute by ID.
pub async fn get_dispute(
    client: &ReadOnlyClient,
    profile_id: i64,
    dispute_id: &str,
) -> Result<String, wise_client::error::Error> {
    let dispute = client.disputes().get(profile_id, dispute_id).await?;
    Ok(serde_json::to_string_pretty(&dispute).unwrap_or_else(|_| format!("{:?}", dispute)))
}

/// Get available dispute reasons.
pub async fn get_dispute_reasons(
    client: &ReadOnlyClient,
    profile_id: i64,
) -> Result<String, wise_client::error::Error> {
    let reasons = client.disputes().get_reasons(profile_id).await?;
    Ok(serde_json::to_string_pretty(&reasons).unwrap_or_else(|_| format!("{:?}", reasons)))
}

/// Get available card programs.
pub async fn get_card_availability(
    client: &ReadOnlyClient,
    profile_id: i64,
) -> Result<String, wise_client::error::Error> {
    let programs = client.card_orders().get_availability(profile_id).await?;
    Ok(serde_json::to_string_pretty(&programs).unwrap_or_else(|_| format!("{:?}", programs)))
}

/// Get card spending permissions.
pub async fn get_card_permissions(
    client: &ReadOnlyClient,
    profile_id: i64,
    card_token: &str,
) -> Result<String, wise_client::error::Error> {
    let permissions = client.cards().get_permissions(profile_id, card_token).await?;
    Ok(serde_json::to_string_pretty(&permissions).unwrap_or_else(|_| format!("{:?}", permissions)))
}

/// List supported currencies.
pub async fn list_currencies(
    client: &ReadOnlyClient,
) -> Result<String, wise_client::error::Error> {
    let currencies = client.currencies().list().await?;
    Ok(serde_json::to_string_pretty(&currencies).unwrap_or_else(|_| format!("{:?}", currencies)))
}

/// Get a quote by ID.
pub async fn get_quote(
    client: &ReadOnlyClient,
    profile_id: i64,
    quote_id: &str,
) -> Result<String, wise_client::error::Error> {
    let uuid = uuid::Uuid::parse_str(quote_id)
        .map_err(|e| wise_client::error::Error::Config(format!("Invalid UUID: {}", e)))?;
    let quote = client.quotes().get(profile_id, uuid).await?;
    Ok(serde_json::to_string_pretty(&quote).unwrap_or_else(|_| format!("{:?}", quote)))
}

/// Get a recipient by ID.
pub async fn get_recipient(
    client: &ReadOnlyClient,
    account_id: i64,
) -> Result<String, wise_client::error::Error> {
    let recipient = client.recipients().get(account_id).await?;
    Ok(serde_json::to_string_pretty(&recipient).unwrap_or_else(|_| format!("{:?}", recipient)))
}

/// Get a balance by ID.
pub async fn get_balance(
    client: &ReadOnlyClient,
    profile_id: i64,
    balance_id: i64,
) -> Result<String, wise_client::error::Error> {
    let balance = client.balances().get(profile_id, balance_id).await?;
    Ok(serde_json::to_string_pretty(&balance).unwrap_or_else(|_| format!("{:?}", balance)))
}

/// Get a profile by ID.
pub async fn get_profile(
    client: &ReadOnlyClient,
    profile_id: i64,
) -> Result<String, wise_client::error::Error> {
    let profile = client.profiles().get(profile_id).await?;
    Ok(serde_json::to_string_pretty(&profile).unwrap_or_else(|_| format!("{:?}", profile)))
}
