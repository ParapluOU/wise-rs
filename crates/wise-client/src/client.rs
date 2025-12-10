//! HTTP client implementations for the Wise API.

use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use secrecy::ExposeSecret;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::api::{
    ActivitiesApi, AddressesApi, AddressesApiMut, BalancesApi, BalancesApiMut, BankDetailsApi,
    BankDetailsApiMut, BatchGroupsApi, BatchGroupsApiMut, CardOrdersApi, CardOrdersApiMut,
    CardTransactionsApi, CardsApi, CardsApiMut, CasesApi, CasesApiMut, ContactsApiMut,
    CurrenciesApi, DirectDebitsApi, DirectDebitsApiMut, DisputesApi, DisputesApiMut, ProfilesApi,
    ProfilesApiMut, QuotesApi, QuotesApiMut, RatesApi, RecipientsApi, RecipientsApiMut,
    SettlementsApiMut, StatementsApi, ThreeDSecureApiMut, TransfersApi, TransfersApiMut, UserApi,
    UserApiMut,
};
use crate::config::{AuthConfig, ClientConfig};
use crate::error::{ApiErrorResponse, Error, Result};

/// Internal client shared between ReadOnlyClient and FullClient.
pub(crate) struct ClientInner {
    http: reqwest::Client,
    base_url: String,
}

impl ClientInner {
    fn new(config: &ClientConfig) -> Result<Self> {
        let mut headers = HeaderMap::new();

        // Set authorization header
        match &config.auth {
            AuthConfig::ApiToken(token) => {
                let auth_value = format!("Bearer {}", token.expose_secret());
                headers.insert(
                    AUTHORIZATION,
                    HeaderValue::from_str(&auth_value).map_err(|e| Error::Config(e.to_string()))?,
                );
            }
        }

        // Set content type
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        // Set user agent
        let user_agent = config
            .user_agent
            .clone()
            .unwrap_or_else(|| format!("wise-rs/{}", env!("CARGO_PKG_VERSION")));
        headers.insert(
            USER_AGENT,
            HeaderValue::from_str(&user_agent).map_err(|e| Error::Config(e.to_string()))?,
        );

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(config.timeout)
            .build()?;

        Ok(Self {
            http,
            base_url: config.environment.base_url().to_string(),
        })
    }

    /// Perform a GET request.
    pub(crate) async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        tracing::debug!("GET {}", url);

        let response = self.http.get(&url).send().await?;
        self.handle_response(response).await
    }

    /// Perform a POST request.
    pub(crate) async fn post<T: DeserializeOwned, B: Serialize + ?Sized>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        tracing::debug!("POST {}", url);

        let response = self.http.post(&url).json(body).send().await?;
        self.handle_response(response).await
    }

    /// Perform a PUT request.
    pub(crate) async fn put<T: DeserializeOwned, B: Serialize + ?Sized>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        tracing::debug!("PUT {}", url);

        let response = self.http.put(&url).json(body).send().await?;
        self.handle_response(response).await
    }

    /// Perform a PATCH request.
    pub(crate) async fn patch<T: DeserializeOwned, B: Serialize + ?Sized>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        tracing::debug!("PATCH {}", url);

        let response = self.http.patch(&url).json(body).send().await?;
        self.handle_response(response).await
    }

    /// Perform a DELETE request.
    pub(crate) async fn delete(&self, path: &str) -> Result<()> {
        let url = format!("{}{}", self.base_url, path);
        tracing::debug!("DELETE {}", url);

        let response = self.http.delete(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await.unwrap_or_default();
            self.handle_error(status.as_u16(), &error_body)
        }
    }

    async fn handle_response<T: DeserializeOwned>(&self, response: reqwest::Response) -> Result<T> {
        let status = response.status();

        if status.is_success() {
            let body = response.text().await?;
            tracing::trace!("Response body: {}", body);
            serde_json::from_str(&body).map_err(Error::from)
        } else {
            let error_body = response.text().await.unwrap_or_default();
            self.handle_error(status.as_u16(), &error_body)
        }
    }

    fn handle_error<T>(&self, status: u16, body: &str) -> Result<T> {
        tracing::debug!("API error ({}): {}", status, body);

        // Handle rate limiting
        if status == 429 {
            return Err(Error::RateLimit {
                retry_after_secs: None,
            });
        }

        // Try to parse error response
        let (message, errors) = if let Ok(err) = serde_json::from_str::<ApiErrorResponse>(body) {
            err.into()
        } else {
            (body.to_string(), vec![])
        };

        Err(Error::Api {
            status,
            message,
            errors,
        })
    }
}

/// Read-only client for the Wise API.
///
/// This client only exposes GET endpoints, making it safe to use in contexts
/// where write operations should be prevented (e.g., MCP tools in read-only mode).
///
/// # Example
/// ```no_run
/// use wise_client::{ReadOnlyClient, ClientConfig};
///
/// # async fn example() -> wise_client::error::Result<()> {
/// let client = ReadOnlyClient::new(ClientConfig::with_token("your-api-token").sandbox())?;
///
/// // Read operations are available
/// let user = client.user().get().await?;
/// let profiles = client.profiles().list().await?;
///
/// // Write operations are NOT available on ReadOnlyClient
/// // client.transfers().create(...) // This won't compile!
/// # Ok(())
/// # }
/// ```
pub struct ReadOnlyClient {
    inner: ClientInner,
}

impl ReadOnlyClient {
    /// Create a new read-only client.
    pub fn new(config: ClientConfig) -> Result<Self> {
        Ok(Self {
            inner: ClientInner::new(&config)?,
        })
    }

    /// Access user endpoints.
    pub fn user(&self) -> UserApi<'_> {
        UserApi {
            client: &self.inner,
        }
    }

    /// Access profile endpoints (read-only).
    pub fn profiles(&self) -> ProfilesApi<'_> {
        ProfilesApi {
            client: &self.inner,
        }
    }

    /// Access balance endpoints (read-only).
    pub fn balances(&self) -> BalancesApi<'_> {
        BalancesApi {
            client: &self.inner,
        }
    }

    /// Access transfer endpoints (read-only).
    pub fn transfers(&self) -> TransfersApi<'_> {
        TransfersApi {
            client: &self.inner,
        }
    }

    /// Access quote endpoints (read-only).
    pub fn quotes(&self) -> QuotesApi<'_> {
        QuotesApi {
            client: &self.inner,
        }
    }

    /// Access recipient endpoints (read-only).
    pub fn recipients(&self) -> RecipientsApi<'_> {
        RecipientsApi {
            client: &self.inner,
        }
    }

    /// Access exchange rate endpoints.
    pub fn rates(&self) -> RatesApi<'_> {
        RatesApi {
            client: &self.inner,
        }
    }

    /// Access currency endpoints.
    pub fn currencies(&self) -> CurrenciesApi<'_> {
        CurrenciesApi {
            client: &self.inner,
        }
    }

    /// Access activity endpoints (read-only).
    pub fn activities(&self) -> ActivitiesApi<'_> {
        ActivitiesApi {
            client: &self.inner,
        }
    }

    /// Access address endpoints (read-only).
    pub fn addresses(&self) -> AddressesApi<'_> {
        AddressesApi {
            client: &self.inner,
        }
    }

    /// Access balance statement endpoints (read-only).
    pub fn statements(&self) -> StatementsApi<'_> {
        StatementsApi {
            client: &self.inner,
        }
    }

    /// Access bank account details endpoints (read-only).
    pub fn bank_details(&self) -> BankDetailsApi<'_> {
        BankDetailsApi {
            client: &self.inner,
        }
    }

    /// Access batch group endpoints (read-only).
    pub fn batch_groups(&self) -> BatchGroupsApi<'_> {
        BatchGroupsApi {
            client: &self.inner,
        }
    }

    /// Access card endpoints (read-only).
    pub fn cards(&self) -> CardsApi<'_> {
        CardsApi {
            client: &self.inner,
        }
    }

    /// Access card order endpoints (read-only).
    pub fn card_orders(&self) -> CardOrdersApi<'_> {
        CardOrdersApi {
            client: &self.inner,
        }
    }

    /// Access card transaction endpoints (read-only).
    pub fn card_transactions(&self) -> CardTransactionsApi<'_> {
        CardTransactionsApi {
            client: &self.inner,
        }
    }

    /// Access partner cases endpoints (read-only).
    pub fn cases(&self) -> CasesApi<'_> {
        CasesApi {
            client: &self.inner,
        }
    }

    /// Access direct debit endpoints (read-only).
    pub fn direct_debits(&self) -> DirectDebitsApi<'_> {
        DirectDebitsApi {
            client: &self.inner,
        }
    }

    /// Access disputes endpoints (read-only).
    pub fn disputes(&self) -> DisputesApi<'_> {
        DisputesApi {
            client: &self.inner,
        }
    }
}

/// Full-access client for the Wise API.
///
/// This client exposes all endpoints including write operations (create, update, delete).
/// Use with caution as write operations can transfer real money in production.
///
/// # Example
/// ```no_run
/// use wise_client::{FullClient, ClientConfig};
/// use wise_client::models::CreateQuote;
/// use rust_decimal_macros::dec;
///
/// # async fn example() -> wise_client::error::Result<()> {
/// let client = FullClient::new(ClientConfig::with_token("your-api-token").sandbox())?;
///
/// // Read operations
/// let profiles = client.profiles().list().await?;
/// let profile_id = profiles[0].id;
///
/// // Write operations
/// let quote = client.quotes().create(
///     profile_id,
///     &CreateQuote::with_source_amount("GBP", "USD", dec!(100.00))
/// ).await?;
/// # Ok(())
/// # }
/// ```
pub struct FullClient {
    inner: ClientInner,
}

impl FullClient {
    /// Create a new full-access client.
    pub fn new(config: ClientConfig) -> Result<Self> {
        Ok(Self {
            inner: ClientInner::new(&config)?,
        })
    }

    /// Access user endpoints (full access).
    pub fn user(&self) -> UserApiMut<'_> {
        UserApiMut {
            client: &self.inner,
        }
    }

    /// Access profile endpoints (full access).
    pub fn profiles(&self) -> ProfilesApiMut<'_> {
        ProfilesApiMut {
            client: &self.inner,
        }
    }

    /// Access balance endpoints (full access).
    pub fn balances(&self) -> BalancesApiMut<'_> {
        BalancesApiMut {
            client: &self.inner,
        }
    }

    /// Access transfer endpoints (full access).
    pub fn transfers(&self) -> TransfersApiMut<'_> {
        TransfersApiMut {
            client: &self.inner,
        }
    }

    /// Access quote endpoints (full access).
    pub fn quotes(&self) -> QuotesApiMut<'_> {
        QuotesApiMut {
            client: &self.inner,
        }
    }

    /// Access recipient endpoints (full access).
    pub fn recipients(&self) -> RecipientsApiMut<'_> {
        RecipientsApiMut {
            client: &self.inner,
        }
    }

    /// Access exchange rate endpoints.
    pub fn rates(&self) -> RatesApi<'_> {
        RatesApi {
            client: &self.inner,
        }
    }

    /// Access currency endpoints.
    pub fn currencies(&self) -> CurrenciesApi<'_> {
        CurrenciesApi {
            client: &self.inner,
        }
    }

    /// Access activity endpoints (read-only).
    pub fn activities(&self) -> ActivitiesApi<'_> {
        ActivitiesApi {
            client: &self.inner,
        }
    }

    /// Access address endpoints (full access).
    pub fn addresses(&self) -> AddressesApiMut<'_> {
        AddressesApiMut {
            client: &self.inner,
        }
    }

    /// Access balance statement endpoints (read-only).
    pub fn statements(&self) -> StatementsApi<'_> {
        StatementsApi {
            client: &self.inner,
        }
    }

    /// Access bank account details endpoints (full access).
    pub fn bank_details(&self) -> BankDetailsApiMut<'_> {
        BankDetailsApiMut {
            client: &self.inner,
        }
    }

    /// Access batch group endpoints (full access).
    pub fn batch_groups(&self) -> BatchGroupsApiMut<'_> {
        BatchGroupsApiMut {
            client: &self.inner,
        }
    }

    /// Access bulk settlement endpoints (full access).
    pub fn settlements(&self) -> SettlementsApiMut<'_> {
        SettlementsApiMut {
            client: &self.inner,
        }
    }

    /// Access card endpoints (full access).
    pub fn cards(&self) -> CardsApiMut<'_> {
        CardsApiMut {
            client: &self.inner,
        }
    }

    /// Access card order endpoints (full access).
    pub fn card_orders(&self) -> CardOrdersApiMut<'_> {
        CardOrdersApiMut {
            client: &self.inner,
        }
    }

    /// Access card transaction endpoints (read-only).
    pub fn card_transactions(&self) -> CardTransactionsApi<'_> {
        CardTransactionsApi {
            client: &self.inner,
        }
    }

    /// Access 3D Secure endpoints (full access).
    pub fn three_d_secure(&self) -> ThreeDSecureApiMut<'_> {
        ThreeDSecureApiMut {
            client: &self.inner,
        }
    }

    /// Access partner cases endpoints (full access).
    pub fn cases(&self) -> CasesApiMut<'_> {
        CasesApiMut {
            client: &self.inner,
        }
    }

    /// Access contact endpoints (full access).
    pub fn contacts(&self) -> ContactsApiMut<'_> {
        ContactsApiMut {
            client: &self.inner,
        }
    }

    /// Access direct debit endpoints (full access).
    pub fn direct_debits(&self) -> DirectDebitsApiMut<'_> {
        DirectDebitsApiMut {
            client: &self.inner,
        }
    }

    /// Access disputes endpoints (full access).
    pub fn disputes(&self) -> DisputesApiMut<'_> {
        DisputesApiMut {
            client: &self.inner,
        }
    }

    /// Get a read-only view of this client.
    ///
    /// Useful when you need to pass the client to code that should only read.
    pub fn as_read_only(&self) -> ReadOnlyClientRef<'_> {
        ReadOnlyClientRef {
            inner: &self.inner,
        }
    }
}

/// A borrowed read-only reference to a client.
///
/// This allows passing a full client to code that should only perform read operations.
pub struct ReadOnlyClientRef<'a> {
    inner: &'a ClientInner,
}

impl<'a> ReadOnlyClientRef<'a> {
    /// Access user endpoints.
    pub fn user(&self) -> UserApi<'_> {
        UserApi {
            client: self.inner,
        }
    }

    /// Access profile endpoints (read-only).
    pub fn profiles(&self) -> ProfilesApi<'_> {
        ProfilesApi {
            client: self.inner,
        }
    }

    /// Access balance endpoints (read-only).
    pub fn balances(&self) -> BalancesApi<'_> {
        BalancesApi {
            client: self.inner,
        }
    }

    /// Access transfer endpoints (read-only).
    pub fn transfers(&self) -> TransfersApi<'_> {
        TransfersApi {
            client: self.inner,
        }
    }

    /// Access quote endpoints (read-only).
    pub fn quotes(&self) -> QuotesApi<'_> {
        QuotesApi {
            client: self.inner,
        }
    }

    /// Access recipient endpoints (read-only).
    pub fn recipients(&self) -> RecipientsApi<'_> {
        RecipientsApi {
            client: self.inner,
        }
    }

    /// Access exchange rate endpoints.
    pub fn rates(&self) -> RatesApi<'_> {
        RatesApi {
            client: self.inner,
        }
    }

    /// Access currency endpoints.
    pub fn currencies(&self) -> CurrenciesApi<'_> {
        CurrenciesApi {
            client: self.inner,
        }
    }

    /// Access activity endpoints (read-only).
    pub fn activities(&self) -> ActivitiesApi<'_> {
        ActivitiesApi {
            client: self.inner,
        }
    }

    /// Access address endpoints (read-only).
    pub fn addresses(&self) -> AddressesApi<'_> {
        AddressesApi {
            client: self.inner,
        }
    }

    /// Access balance statement endpoints (read-only).
    pub fn statements(&self) -> StatementsApi<'_> {
        StatementsApi {
            client: self.inner,
        }
    }

    /// Access bank account details endpoints (read-only).
    pub fn bank_details(&self) -> BankDetailsApi<'_> {
        BankDetailsApi {
            client: self.inner,
        }
    }

    /// Access batch group endpoints (read-only).
    pub fn batch_groups(&self) -> BatchGroupsApi<'_> {
        BatchGroupsApi {
            client: self.inner,
        }
    }

    /// Access card endpoints (read-only).
    pub fn cards(&self) -> CardsApi<'_> {
        CardsApi {
            client: self.inner,
        }
    }

    /// Access card order endpoints (read-only).
    pub fn card_orders(&self) -> CardOrdersApi<'_> {
        CardOrdersApi {
            client: self.inner,
        }
    }

    /// Access card transaction endpoints (read-only).
    pub fn card_transactions(&self) -> CardTransactionsApi<'_> {
        CardTransactionsApi {
            client: self.inner,
        }
    }

    /// Access partner cases endpoints (read-only).
    pub fn cases(&self) -> CasesApi<'_> {
        CasesApi {
            client: self.inner,
        }
    }

    /// Access direct debit endpoints (read-only).
    pub fn direct_debits(&self) -> DirectDebitsApi<'_> {
        DirectDebitsApi {
            client: self.inner,
        }
    }

    /// Access disputes endpoints (read-only).
    pub fn disputes(&self) -> DisputesApi<'_> {
        DisputesApi {
            client: self.inner,
        }
    }
}
