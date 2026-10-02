//! NOWPayments API type definitions

use serde::{Deserialize, Serialize};

/// Payment status values – API may return "sending" or "spending"
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentStatus {
    Waiting,
    Confirming,
    Confirmed,
    Spending,
    Sending,
    PartiallyPaid,
    Finished,
    Failed,
    Refunded,
    Expired,
}

impl PaymentStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Waiting => "waiting",
            Self::Confirming => "confirming",
            Self::Confirmed => "confirmed",
            Self::Spending => "spending",
            Self::Sending => "sending",
            Self::PartiallyPaid => "partially_paid",
            Self::Finished => "finished",
            Self::Failed => "failed",
            Self::Refunded => "refunded",
            Self::Expired => "expired",
        }
    }
}

impl std::fmt::Display for PaymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for PaymentStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "waiting" => Ok(Self::Waiting),
            "confirming" => Ok(Self::Confirming),
            "confirmed" => Ok(Self::Confirmed),
            "spending" => Ok(Self::Spending),
            "sending" => Ok(Self::Sending),
            "partially_paid" => Ok(Self::PartiallyPaid),
            "finished" => Ok(Self::Finished),
            "failed" => Ok(Self::Failed),
            "refunded" => Ok(Self::Refunded),
            "expired" => Ok(Self::Expired),
            _ => Err(()),
        }
    }
}

/// All possible payment statuses
pub static PAYMENT_STATUSES: &[&str] = &[
    "waiting", "confirming", "confirmed", "spending", "sending", "partially_paid", "finished",
    "failed", "refunded", "expired",
];

/// Statuses that mean payment is done (success or terminal)
pub static PAYMENT_DONE_STATUSES: &[&str] = &["finished", "failed", "refunded", "expired"];

/// Statuses that mean customer should still pay
pub static PAYMENT_PENDING_STATUSES: &[&str] = &[
    "waiting", "confirming", "confirmed", "spending", "sending", "partially_paid",
];

/// Client configuration options
#[derive(Debug, Clone)]
pub struct NowPaymentsConfig {
    /// Your NOWPayments API key (required)
    pub api_key: String,
    /// Use sandbox API (api-sandbox.nowpayments.io)
    pub sandbox: Option<bool>,
    /// Custom base URL (overrides sandbox)
    pub base_url: Option<String>,
    /// Request timeout in milliseconds (default: 30000)
    pub timeout: Option<u64>,
    /// IPN secret for verifying webhook callbacks
    pub ipn_secret: Option<String>,
}

impl Default for NowPaymentsConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            sandbox: Some(false),
            base_url: None,
            timeout: Some(30000),
            ipn_secret: None,
        }
    }
}

/// Currencies list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrenciesResponse {
    pub currencies: Vec<String>,
}

/// Full currencies list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullCurrenciesResponse {
    pub currencies: Vec<FullCurrency>,
}

/// Update payment estimate response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdatePaymentEstimateResponse {
    pub pay_amount: f64,
    pub expiration_estimate_date: String,
    pub id: String,
    pub token_id: String,
}

/// Create payment request
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CreatePaymentParams {
    pub price_amount: f64,
    pub price_currency: String,
    pub pay_currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipn_callback_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_extra_id: Option<String>,
    /// Deprecated alias for `is_fixed_rate` (mapped at request time).
    #[serde(skip_serializing)]
    pub fixed_rate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_fixed_rate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_fee_paid_by_user: Option<bool>,
    /// Customer IP for fiat2crypto (`origin-ip` header). Not sent in JSON body.
    #[serde(skip_serializing)]
    pub origin_ip: Option<String>,
}

/// Create invoice request
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CreateInvoiceParams {
    pub price_amount: f64,
    pub price_currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipn_callback_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partially_paid_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_fixed_rate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_fee_paid_by_user: Option<bool>,
    /// Customer IP for fiat2crypto (`origin-ip` header). Not sent in JSON body.
    #[serde(skip_serializing)]
    pub origin_ip: Option<String>,
}

/// Full currency details from GET /v1/full-currencies
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FullCurrency {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub enable: bool,
    #[serde(default)]
    pub wallet_regex: Option<String>,
    #[serde(default)]
    pub priority: Option<i64>,
    #[serde(default)]
    pub extra_id_exists: Option<bool>,
    #[serde(default)]
    pub extra_id_regex: Option<String>,
    #[serde(default)]
    pub logo_url: Option<String>,
    #[serde(default)]
    pub track: Option<bool>,
    #[serde(default)]
    pub cg_id: Option<String>,
    #[serde(default)]
    pub is_maxlimit: Option<bool>,
    #[serde(default)]
    pub network: Option<String>,
    #[serde(default)]
    pub smart_contract: Option<String>,
    #[serde(default)]
    pub network_precision: Option<i64>,
}

/// Fiat payout crypto currency option
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FiatPayoutCryptoCurrency {
    pub provider: String,
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
    #[serde(rename = "currencyNetwork")]
    pub currency_network: String,
    pub enabled: bool,
}

/// Fiat payout payment method field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FiatPayoutField {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
    pub mandatory: bool,
    #[serde(default)]
    pub description: Option<String>,
}

/// Fiat payout payment method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FiatPayoutPaymentMethod {
    pub name: String,
    #[serde(rename = "paymentCode")]
    pub payment_code: String,
    pub fields: Vec<FiatPayoutField>,
    pub provider: String,
}

/// Fiat payout record
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FiatPayoutRecord {
    pub id: String,
    pub provider: String,
    pub request_id: String,
    pub status: String,
    #[serde(default)]
    pub fiat_currency_code: Option<String>,
    #[serde(default)]
    pub fiat_amount: Option<String>,
    #[serde(default)]
    pub crypto_currency_code: Option<String>,
    #[serde(default)]
    pub crypto_currency_amount: Option<String>,
    #[serde(default)]
    pub fiat_account_code: Option<String>,
    #[serde(default)]
    pub fiat_account_number: Option<String>,
    #[serde(default)]
    pub payout_description: Option<String>,
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

/// List subscription plans params
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GetSubscriptionPlansParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

/// List subscriptions params
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GetSubscriptionsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_plan_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

/// List fiat payouts params
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFiatPayoutsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fiat_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crypto_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_payout_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_to: Option<String>,
}

/// Payment object from API
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Payment {
    #[serde(alias = "payment_id")]
    pub payment_id: serde_json::Value,
    pub payment_status: String,
    pub pay_address: String,
    pub pay_amount: f64,
    pub pay_currency: String,
    pub price_amount: f64,
    pub price_currency: String,
    #[serde(default)]
    pub actually_paid: Option<f64>,
    #[serde(default)]
    pub outcome_amount: Option<f64>,
    #[serde(default)]
    pub outcome_currency: Option<String>,
    #[serde(default)]
    pub order_id: Option<String>,
    #[serde(default)]
    pub order_description: Option<String>,
    #[serde(default)]
    pub purchase_id: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

/// Paginated payments list
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentsListResponse {
    pub data: Vec<Payment>,
    pub limit: u32,
    pub page: u32,
    pub pages_count: u32,
    pub total: u32,
}

/// Estimate price response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EstimatePriceResponse {
    pub amount_from: f64,
    pub currency_from: String,
    pub currency_to: String,
    pub estimated_amount: f64,
}

/// Minimum amount response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MinAmountResponse {
    pub currency_from: String,
    pub currency_to: String,
    pub min_amount: f64,
    #[serde(default)]
    pub fiat_equivalent: Option<f64>,
}

/// List payments query params
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListPaymentsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_to: Option<String>,
}

/// Get estimate params
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EstimateParams {
    pub amount: f64,
    pub currency_from: String,
    pub currency_to: String,
}

/// Get min amount params
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct MinAmountParams {
    pub currency_from: String,
    pub currency_to: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fiat_equivalent: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_fixed_rate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_fee_paid_by_user: Option<bool>,
}

/// Invoice response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InvoiceResponse {
    pub id: String,
    #[serde(default)]
    pub invoice_id: Option<String>,
    pub invoice_url: String,
    pub price_amount: f64,
    pub price_currency: String,
    #[serde(default)]
    pub pay_currency: Option<String>,
    #[serde(default)]
    pub order_id: Option<String>,
    #[serde(default)]
    pub order_description: Option<String>,
}

/// Subscription plan
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SubscriptionPlan {
    pub id: String,
    pub amount: f64,
    pub currency: String,
    pub interval_day: String,
    pub title: String,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

/// Subscriber info
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Subscriber {
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub sub_partner_id: Option<String>,
}

/// Recurring payment/subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RecurringPayment {
    pub id: String,
    pub subscription_plan_id: String,
    pub status: String,
    pub is_active: bool,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub expire_date: Option<String>,
    #[serde(default)]
    pub subscriber: Option<Subscriber>,
}

/// Balance entry (nested in SubPartnerBalance, API returns camelCase)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceEntry {
    pub amount: f64,
    pub pending_amount: f64,
}

/// Sub-partner balance
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubPartnerBalance {
    pub sub_partner_id: String,
    pub balances: std::collections::HashMap<String, BalanceEntry>,
}

/// Payout withdrawal item for createPayout
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct PayoutWithdrawal {
    pub address: String,
    pub currency: String,
    pub amount: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipn_callback_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fiat_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fiat_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execute_at: Option<String>,
}

/// Create payout request body
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreatePayoutParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipn_callback_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_description: Option<String>,
    pub withdrawals: Vec<PayoutWithdrawal>,
}

/// Payout withdrawal response item
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PayoutWithdrawalResponse {
    pub id: String,
    pub address: String,
    pub currency: String,
    pub amount: String,
    pub status: String,
    pub batch_withdrawal_id: String,
}

/// Create payout response (batch)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePayoutResponse {
    pub id: String,
    pub withdrawals: Vec<PayoutWithdrawalResponse>,
}

/// Auth response (JWT token for payouts, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
}

/// Validate address params
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidateAddressParams {
    pub address: String,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_id: Option<String>,
}

/// Create sub-partner deposit payment (top up sub-partner balance)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateSubPartnerPaymentParams {
    pub currency: String,
    pub amount: f64,
    pub sub_partner_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_rate: Option<bool>,
}

/// Sub-partner payment response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubPartnerPaymentResponse {
    pub result: Payment,
}

/// Create payment for existing invoice
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CreateInvoicePaymentParams {
    pub iid: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_extra_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_currency: Option<String>,
}

/// API status response (GET /v1/status)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiStatusResponse {
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
}

/// Subscriptions list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionsResponse {
    pub count: u32,
    pub result: Vec<RecurringPayment>,
}

/// Single subscription response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionResult {
    pub result: RecurringPayment,
}

/// Delete subscription response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSubscriptionResponse {
    pub result: String,
}

/// Subscription plans list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionPlansResponse {
    pub count: u32,
    pub result: Vec<SubscriptionPlan>,
}

/// Single subscription plan response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionPlanResult {
    pub result: SubscriptionPlan,
}

/// Create subscription params
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateSubscriptionParams {
    pub subscription_plan_id: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_partner_id: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

/// Sub-partners list params
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubPartnersParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}

/// Transfers list params
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransfersParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}

/// Sub-partner balance response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubPartnerBalanceResult {
    pub result: SubPartnerBalance,
}

/// Fiat payouts response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FiatPayoutsResponse {
    pub result: FiatPayoutsResult,
}

/// Fiat payouts result with rows
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FiatPayoutsResult {
    pub rows: Vec<FiatPayoutRecord>,
}

/// List payouts query params
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GetPayoutsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
}

/// Fiat payout crypto currencies / payment methods filter
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FiatPayoutsCryptoParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

/// Fiat payout crypto currencies response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FiatPayoutCryptoCurrenciesResponse {
    pub result: Vec<FiatPayoutCryptoCurrency>,
}

/// Fiat payout payment methods response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FiatPayoutPaymentMethodsResponse {
    pub result: Vec<FiatPayoutPaymentMethod>,
}

/// Create sub-partner response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubPartnerResponse {
    pub result: CreateSubPartnerResult,
}

/// Create sub-partner result body
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateSubPartnerResult {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

/// Create subscription response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubscriptionResponse {
    pub result: RecurringPayment,
}

/// Transfer between sub-partner accounts
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateTransferParams {
    pub currency: String,
    pub amount: f64,
    pub from_id: serde_json::Value,
    pub to_id: serde_json::Value,
}

/// Write-off from sub-partner to master
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WriteOffParams {
    pub currency: String,
    pub amount: f64,
    pub sub_partner_id: serde_json::Value,
}

/// Deposit from master to sub-partner
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DepositParams {
    pub currency: String,
    pub amount: f64,
    pub sub_partner_id: serde_json::Value,
}

/// Create conversion params
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateConversionParams {
    pub amount: f64,
    pub from_currency: String,
    pub to_currency: String,
}

/// List conversions params
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GetConversionsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}
