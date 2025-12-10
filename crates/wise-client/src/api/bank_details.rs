//! Bank account details API endpoints.

use crate::client::ClientInner;
use crate::error::Result;
use crate::models::{
    BankAccountDetails, BankDetailsOrder, CreateBankDetailsOrderRequest,
    CreateMultipleBankDetailsRequest, CreatePaymentReturnRequest, MultipleBankDetailsResponse,
    PaymentReturnResponse,
};

/// Read-only Bank Account Details API operations.
pub struct BankDetailsApi<'a> {
    pub(crate) client: &'a ClientInner,
}

impl<'a> BankDetailsApi<'a> {
    /// Get bank account details for a profile.
    ///
    /// Returns all AVAILABLE and ACTIVE account details, including examples.
    pub async fn list(&self, profile_id: i64) -> Result<Vec<BankAccountDetails>> {
        self.client
            .get(&format!("/v1/profiles/{}/account-details", profile_id))
            .await
    }

    /// List bank account detail orders.
    pub async fn list_orders(
        &self,
        profile_id: i64,
        currency: &str,
    ) -> Result<Vec<BankDetailsOrder>> {
        self.client
            .get(&format!(
                "/v3/profiles/{}/account-details-orders?currency={}",
                profile_id, currency
            ))
            .await
    }
}

/// Full Bank Account Details API operations (includes write operations).
pub struct BankDetailsApiMut<'a> {
    pub(crate) client: &'a ClientInner,
}

impl<'a> BankDetailsApiMut<'a> {
    /// Get bank account details for a profile.
    pub async fn list(&self, profile_id: i64) -> Result<Vec<BankAccountDetails>> {
        self.client
            .get(&format!("/v1/profiles/{}/account-details", profile_id))
            .await
    }

    /// List bank account detail orders.
    pub async fn list_orders(
        &self,
        profile_id: i64,
        currency: &str,
    ) -> Result<Vec<BankDetailsOrder>> {
        self.client
            .get(&format!(
                "/v3/profiles/{}/account-details-orders?currency={}",
                profile_id, currency
            ))
            .await
    }

    /// Create a bank account details order.
    ///
    /// Creates an order to issue account details for the specified currency.
    pub async fn create_order(
        &self,
        profile_id: i64,
        currency: &str,
    ) -> Result<BankDetailsOrder> {
        let request = CreateBankDetailsOrderRequest {
            currency: currency.to_string(),
        };
        self.client
            .post(
                &format!("/v1/profiles/{}/account-details-orders", profile_id),
                &request,
            )
            .await
    }

    /// Create multiple bank account details.
    ///
    /// Creates and assigns local and international account details for a balance.
    /// Note: Contact Wise Support Team for access to this endpoint.
    pub async fn create_multiple(
        &self,
        profile_id: i64,
        target_account_id: i64,
    ) -> Result<MultipleBankDetailsResponse> {
        let request = CreateMultipleBankDetailsRequest { target_account_id };
        self.client
            .post(&format!("/v3/profiles/{}/bank-details", profile_id), &request)
            .await
    }

    /// Create a return for a payment received to bank account details.
    ///
    /// The reason is required for Swift payments.
    pub async fn create_payment_return(
        &self,
        profile_id: i64,
        payment_id: &str,
        request: &CreatePaymentReturnRequest,
    ) -> Result<PaymentReturnResponse> {
        self.client
            .post(
                &format!(
                    "/v1/profiles/{}/account-details/payments/{}/returns",
                    profile_id, payment_id
                ),
                request,
            )
            .await
    }
}
