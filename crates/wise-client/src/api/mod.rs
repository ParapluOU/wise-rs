//! API endpoint implementations organized by resource.

mod balances;
mod currencies;
mod profiles;
mod quotes;
mod rates;
mod recipients;
mod transfers;
mod user;

pub use balances::{BalancesApi, BalancesApiMut};
pub use currencies::CurrenciesApi;
pub use profiles::{ProfilesApi, ProfilesApiMut};
pub use quotes::{QuotesApi, QuotesApiMut};
pub use rates::RatesApi;
pub use recipients::{RecipientsApi, RecipientsApiMut};
pub use transfers::{TransfersApi, TransfersApiMut};
pub use user::{UserApi, UserApiMut};
