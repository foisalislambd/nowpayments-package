//! NOWPayments API client - main implementation

use crate::error::NowPaymentsError;
use crate::http::HttpClient;
use crate::ipn::verify_ipn_signature;
use crate::types::*;
use reqwest::Method;
use serde_json::Value;
use std::time::Duration;

const PRODUCTION_URL: &str = "https://api.nowpayments.io";
const SANDBOX_URL: &str = "https://api-sandbox.nowpayments.io";

/// NOWPayments API client
pub struct NowPayments {
    client: HttpClient,
    config: NowPaymentsConfig,
}

impl NowPayments {
    /// Create a new NowPayments client
    pub fn new(config: NowPaymentsConfig) -> Result<Self, NowPaymentsError> {
        if config.api_key.trim().is_empty() {
            return Err(NowPaymentsError::new(
                "NOWPayments API key is required. Get yours at https://account.nowpayments.io",
                None,
                None,
                None,
            ));
        }
        let base_url = config
            .base_url
            .clone()
            .unwrap_or_else(|| {
                if config.sandbox.unwrap_or(false) {
                    SANDBOX_URL.to_string()
                } else {
                    PRODUCTION_URL.to_string()
                }
            });
        let timeout = config
            .timeout
            .unwrap_or(30000)
            .min(300000)
            .max(1000) as u64;
        let client = HttpClient::new(
            &base_url,
            &config.api_key,
            Duration::from_millis(timeout),
        )?;
        Ok(Self { client, config })
    }

    /// Check if API is up and available
    pub async fn get_status(&self) -> Result<ApiStatusResponse, NowPaymentsError> {
        self.client.get("/v1/status").await
    }

    /// Get list of available crypto currencies
    pub async fn get_currencies(
        &self,
        fixed_rate: Option<bool>,
    ) -> Result<CurrenciesResponse, NowPaymentsError> {
        let mut params = vec![];
        if let Some(fr) = fixed_rate {
            params.push(("fixed_rate", fr.to_string()));
        }
        self.client
            .get_with_params("/v1/currencies", &params)
            .await
    }

    /// Get full currency details
    pub async fn get_full_currencies(&self) -> Result<FullCurrenciesResponse, NowPaymentsError> {
        self.client.get("/v1/full-currencies").await
    }

    /// Get merchant checked currencies
    pub async fn get_merchant_coins(
        &self,
        fixed_rate: Option<bool>,
    ) -> Result<CurrenciesResponse, NowPaymentsError> {
        let mut params = vec![];
        if let Some(fr) = fixed_rate {
            params.push(("fixed_rate", fr.to_string()));
        }
        self.client
            .get_with_params("/v1/merchant/coins", &params)
            .await
    }

    /// Get JWT token (required for payouts, custody, etc.)
    pub async fn get_auth_token(
        &self,
        email: &str,
        password: &str,
    ) -> Result<AuthResponse, NowPaymentsError> {
        let body = serde_json::json!({ "email": email, "password": password });
        self.client.post("/v1/auth", body).await
    }

    /// Get single currency details
    pub async fn get_currency(&self, currency: &str) -> Result<Value, NowPaymentsError> {
        let code = currency.trim();
        if code.is_empty() {
            return Err(NowPaymentsError::new(
                "Currency code is required (e.g. \"btc\", \"eth\")",
                None,
                None,
                None,
            ));
        }
        self.client.get_raw(&format!("/v1/currencies/{}", code)).await
    }

    /// Get estimated price in crypto for a fiat amount
    pub async fn get_estimate_price(
        &self,
        params: &EstimateParams,
    ) -> Result<EstimatePriceResponse, NowPaymentsError> {
        let query = [
            ("amount", params.amount.to_string()),
            ("currency_from", params.currency_from.clone()),
            ("currency_to", params.currency_to.clone()),
        ];
        self.client
            .get_with_params("/v1/estimate", &query)
            .await
    }

    /// Get minimum payment amount for currency pair
    pub async fn get_min_amount(
        &self,
        params: &MinAmountParams,
    ) -> Result<MinAmountResponse, NowPaymentsError> {
        let mut query: Vec<(&str, String)> = vec![
            ("currency_from", params.currency_from.clone()),
            ("currency_to", params.currency_to.clone()),
        ];
        if let Some(ref fe) = params.fiat_equivalent {
            query.push(("fiat_equivalent", fe.to_string()));
        }
        if let Some(fr) = params.is_fixed_rate {
            query.push(("is_fixed_rate", fr.to_string()));
        }
        if let Some(fu) = params.is_fee_paid_by_user {
            query.push(("is_fee_paid_by_user", fu.to_string()));
        }
        self.client
            .get_with_params("/v1/min-amount", &query)
            .await
    }

    /// Create a new payment
    pub async fn create_payment(&self, params: CreatePaymentParams) -> Result<Payment, NowPaymentsError> {
        let mut body = serde_json::to_value(&params).map_err(|e| {
            NowPaymentsError::new(e.to_string(), None, None, None)
        })?;
        if let Some(obj) = body.as_object_mut() {
            if params.is_fixed_rate.is_none() {
                if let Some(fr) = params.fixed_rate {
                    obj.insert("is_fixed_rate".to_string(), Value::Bool(fr));
                }
            }
        }
        let headers: Vec<(&str, &str)> = params
            .origin_ip
            .as_deref()
            .filter(|ip| !ip.trim().is_empty())
            .map(|ip| vec![("origin-ip", ip)])
            .unwrap_or_default();
        self.client
            .post_with_headers("/v1/payment", body, &headers)
            .await
    }

    /// Get payment status by ID
    pub async fn get_payment_status(
        &self,
        payment_id: &str,
    ) -> Result<Payment, NowPaymentsError> {
        if payment_id.trim().is_empty() {
            return Err(NowPaymentsError::new("Payment ID is required", None, None, None));
        }
        self.client
            .get(&format!("/v1/payment/{}", payment_id))
            .await
    }

    /// Get paginated list of payments. JWT recommended per API docs.
    pub async fn get_payments(
        &self,
        params: Option<&ListPaymentsParams>,
        jwt_token: Option<&str>,
    ) -> Result<PaymentsListResponse, NowPaymentsError> {
        let query: Vec<(&str, String)> = params
            .map(|p| {
                let mut v = vec![];
                if let Some(l) = p.limit {
                    v.push(("limit", l.to_string()));
                }
                if let Some(pg) = p.page {
                    v.push(("page", pg.to_string()));
                }
                if let Some(ref sb) = p.sort_by {
                    v.push(("sortBy", sb.clone()));
                }
                if let Some(ref ob) = p.order_by {
                    v.push(("orderBy", ob.clone()));
                }
                if let Some(ref df) = p.date_from {
                    v.push(("dateFrom", df.clone()));
                }
                if let Some(ref dt) = p.date_to {
                    v.push(("dateTo", dt.clone()));
                }
                v
            })
            .unwrap_or_default();
        match jwt_token {
            Some(token) if !token.trim().is_empty() => {
                self.client
                    .get_with_params_auth("/v1/payment/", &query, Some(token))
                    .await
            }
            _ => self.client.get_with_params("/v1/payment/", &query).await,
        }
    }

    /// Update payment estimate
    pub async fn update_payment_estimate(
        &self,
        payment_id: &str,
    ) -> Result<UpdatePaymentEstimateResponse, NowPaymentsError> {
        self.client
            .request(
                Method::POST,
                &format!("/v1/payment/{}/update-merchant-estimate", payment_id),
                None,
            )
            .await
    }

    /// Create an invoice
    pub async fn create_invoice(
        &self,
        params: CreateInvoiceParams,
    ) -> Result<InvoiceResponse, NowPaymentsError> {
        let body = serde_json::to_value(&params).map_err(|e| {
            NowPaymentsError::new(e.to_string(), None, None, None)
        })?;
        let headers: Vec<(&str, &str)> = params
            .origin_ip
            .as_deref()
            .filter(|ip| !ip.trim().is_empty())
            .map(|ip| vec![("origin-ip", ip)])
            .unwrap_or_default();
        self.client
            .post_with_headers("/v1/invoice", body, &headers)
            .await
    }

    /// Create payment for existing invoice
    pub async fn create_invoice_payment(
        &self,
        params: CreateInvoicePaymentParams,
    ) -> Result<Payment, NowPaymentsError> {
        let body = serde_json::to_value(&params).map_err(|e| {
            NowPaymentsError::new(&e.to_string(), None, None, None)
        })?;
        self.client.post("/v1/invoice-payment", body).await
    }

    // --- Subscriptions ---

    /// List all recurring payments
    pub async fn get_subscriptions(
        &self,
        params: Option<&GetSubscriptionsParams>,
    ) -> Result<SubscriptionsResponse, NowPaymentsError> {
        let query = params.map(|p| {
            let mut v: Vec<(&str, String)> = vec![];
            if let Some(ref s) = p.status {
                v.push(("status", s.clone()));
            }
            if let Some(ref sp) = p.subscription_plan_id {
                v.push(("subscription_plan_id", sp.to_string()));
            }
            if let Some(ia) = p.is_active {
                v.push(("is_active", ia.to_string()));
            }
            if let Some(l) = p.limit {
                v.push(("limit", l.to_string()));
            }
            if let Some(o) = p.offset {
                v.push(("offset", o.to_string()));
            }
            v
        }).unwrap_or_default();
        self.client
            .get_with_params("/v1/subscriptions", &query)
            .await
    }

    /// Get single recurring payment
    pub async fn get_subscription(&self, id: &str) -> Result<SubscriptionResult, NowPaymentsError> {
        self.client
            .get(&format!("/v1/subscriptions/{}", id))
            .await
    }

    /// Cancel recurring payment
    pub async fn delete_subscription(
        &self,
        id: &str,
        jwt_token: Option<&str>,
    ) -> Result<DeleteSubscriptionResponse, NowPaymentsError> {
        self.client
            .delete(&format!("/v1/subscriptions/{}", id), jwt_token)
            .await
    }

    /// List subscription plans
    pub async fn get_subscription_plans(
        &self,
        params: Option<&GetSubscriptionPlansParams>,
    ) -> Result<SubscriptionPlansResponse, NowPaymentsError> {
        let query = params.map(|p| {
            let mut v: Vec<(&str, String)> = vec![];
            if let Some(l) = p.limit {
                v.push(("limit", l.to_string()));
            }
            if let Some(o) = p.offset {
                v.push(("offset", o.to_string()));
            }
            v
        }).unwrap_or_default();
        self.client
            .get_with_params("/v1/subscriptions/plans", &query)
            .await
    }

    /// Get single subscription plan
    pub async fn get_subscription_plan(
        &self,
        id: &str,
    ) -> Result<SubscriptionPlanResult, NowPaymentsError> {
        self.client
            .get(&format!("/v1/subscriptions/plans/{}", id))
            .await
    }

    /// Update subscription plan
    pub async fn update_subscription_plan(
        &self,
        id: &str,
        updates: &serde_json::Value,
    ) -> Result<Value, NowPaymentsError> {
        self.client
            .patch(&format!("/v1/subscriptions/plans/{}", id), updates.clone())
            .await
    }

    // --- Sub-Partners ---

    /// List sub-partners
    pub async fn get_sub_partners(
        &self,
        params: Option<&SubPartnersParams>,
        jwt_token: Option<&str>,
    ) -> Result<Value, NowPaymentsError> {
        let query = params.map(|p| {
            let mut v: Vec<(&str, String)> = vec![];
            if let Some(ref id) = p.id {
                crate::query::push_json_filter_query(&mut v, "id", id);
            }
            if let Some(o) = p.offset {
                v.push(("offset", o.to_string()));
            }
            if let Some(l) = p.limit {
                v.push(("limit", l.to_string()));
            }
            if let Some(ref ord) = p.order {
                v.push(("order", ord.clone()));
            }
            v
        }).unwrap_or_default();
        self.client
            .get_with_params_auth("/v1/sub-partner", &query, jwt_token)
            .await
    }

    /// Get sub-partner balance
    pub async fn get_sub_partner_balance(
        &self,
        sub_partner_id: &str,
    ) -> Result<SubPartnerBalanceResult, NowPaymentsError> {
        self.client
            .get(&format!("/v1/sub-partner/balance/{}", sub_partner_id))
            .await
    }

    /// List transfers
    pub async fn get_transfers(
        &self,
        params: Option<&TransfersParams>,
        jwt_token: Option<&str>,
    ) -> Result<Value, NowPaymentsError> {
        let query = params.map(|p| {
            let mut v: Vec<(&str, String)> = vec![];
            if let Some(ref id) = p.id {
                crate::query::push_json_filter_query(&mut v, "id", id);
            }
            if let Some(ref s) = p.status {
                crate::query::push_json_filter_query(&mut v, "status", s);
            }
            if let Some(l) = p.limit {
                v.push(("limit", l.to_string()));
            }
            if let Some(o) = p.offset {
                v.push(("offset", o.to_string()));
            }
            if let Some(ref ord) = p.order {
                v.push(("order", ord.clone()));
            }
            v
        }).unwrap_or_default();
        self.client
            .get_with_params_auth("/v1/sub-partner/transfers", &query, jwt_token)
            .await
    }

    /// Get single transfer
    pub async fn get_transfer(
        &self,
        id: &str,
        jwt_token: Option<&str>,
    ) -> Result<Value, NowPaymentsError> {
        self.client
            .get_auth(&format!("/v1/sub-partner/transfer/{}", id), jwt_token)
            .await
    }

    // --- Payouts ---

    /// Create mass payout (requires JWT)
    pub async fn create_payout(
        &self,
        params: &CreatePayoutParams,
        jwt_token: &str,
    ) -> Result<CreatePayoutResponse, NowPaymentsError> {
        if jwt_token.trim().is_empty() {
            return Err(NowPaymentsError::new(
                "JWT token is required for createPayout. Call get_auth_token first.",
                None,
                None,
                None,
            ));
        }
        let body = serde_json::to_value(params).map_err(|e| {
            NowPaymentsError::new(&e.to_string(), None, None, None)
        })?;
        self.client
            .post_auth("/v1/payout", body, jwt_token)
            .await
    }

    /// Verify payout with 2FA code
    pub async fn verify_payout(
        &self,
        payout_id: &str,
        verification_code: &str,
        jwt_token: &str,
    ) -> Result<String, NowPaymentsError> {
        if jwt_token.trim().is_empty() {
            return Err(NowPaymentsError::new(
                "JWT token is required for verifyPayout. Call get_auth_token first.",
                None,
                None,
                None,
            ));
        }
        let body = serde_json::json!({ "verification_code": verification_code });
        let text = self
            .client
            .post_auth_text(
                &format!("/v1/payout/{}/verify", payout_id),
                body,
                jwt_token,
            )
            .await?;
        if let Ok(v) = serde_json::from_str::<Value>(&text) {
            if let Some(s) = v.as_str() {
                return Ok(s.to_string());
            }
            if let Some(s) = v.get("result").and_then(|r| r.as_str()) {
                return Ok(s.to_string());
            }
        }
        Ok(text)
    }

    /// Get payout status
    pub async fn get_payout_status(
        &self,
        payout_id: &str,
        jwt_token: Option<&str>,
    ) -> Result<Value, NowPaymentsError> {
        self.client
            .get_auth(&format!("/v1/payout/{}", payout_id), jwt_token)
            .await
    }

    /// List payouts
    pub async fn get_payouts(
        &self,
        params: Option<&GetPayoutsParams>,
    ) -> Result<Value, NowPaymentsError> {
        let query = params.map(|p| {
            let mut v: Vec<(&str, String)> = vec![];
            if let Some(ref bid) = p.batch_id {
                v.push(("batch_id", bid.clone()));
            }
            if let Some(ref s) = p.status {
                v.push(("status", s.clone()));
            }
            if let Some(ref ob) = p.order_by {
                v.push(("order_by", ob.clone()));
            }
            if let Some(ref o) = p.order {
                v.push(("order", o.clone()));
            }
            if let Some(ref df) = p.date_from {
                v.push(("date_from", df.clone()));
            }
            if let Some(ref dt) = p.date_to {
                v.push(("date_to", dt.clone()));
            }
            if let Some(l) = p.limit {
                v.push(("limit", l.to_string()));
            }
            if let Some(pg) = p.page {
                v.push(("page", pg.to_string()));
            }
            v
        }).unwrap_or_default();
        self.client
            .get_with_params("/v1/payout", &query)
            .await
    }

    /// Validate payout address
    pub async fn validate_payout_address(
        &self,
        params: &ValidateAddressParams,
    ) -> Result<Value, NowPaymentsError> {
        let body = serde_json::to_value(params).map_err(|e| {
            NowPaymentsError::new(&e.to_string(), None, None, None)
        })?;
        self.client.post("/v1/payout/validate-address", body).await
    }

    /// Cancel a scheduled payout
    pub async fn cancel_payout(
        &self,
        payout_id: &str,
        jwt_token: &str,
    ) -> Result<(), NowPaymentsError> {
        if jwt_token.trim().is_empty() {
            return Err(NowPaymentsError::new(
                "JWT token is required for cancelPayout. Call get_auth_token first.",
                None,
                None,
                None,
            ));
        }
        let body = serde_json::json!({ "payout_id": payout_id });
        self.client
            .post_auth_no_content("/v1/payout/w_id/cancel", body, jwt_token)
            .await
    }

    /// Estimate network fee for a payout
    pub async fn get_payout_fee(
        &self,
        currency: &str,
        amount: f64,
    ) -> Result<Value, NowPaymentsError> {
        if currency.trim().is_empty() {
            return Err(NowPaymentsError::new(
                "Currency is required (e.g. \"btc\", \"eth\")",
                None,
                None,
                None,
            ));
        }
        let query = [
            ("currency", currency.to_string()),
            ("amount", amount.to_string()),
        ];
        self.client
            .get_with_params("/v1/payout/fee", &query)
            .await
    }

    // --- Fiat Payouts ---

    /// Get crypto currencies for fiat cashout
    pub async fn get_fiat_payouts_crypto_currencies(
        &self,
        params: Option<&FiatPayoutsCryptoParams>,
        jwt_token: Option<&str>,
    ) -> Result<FiatPayoutCryptoCurrenciesResponse, NowPaymentsError> {
        let query = params.map(|p| {
            let mut v: Vec<(&str, String)> = vec![];
            if let Some(ref pr) = p.provider {
                v.push(("provider", pr.clone()));
            }
            if let Some(ref c) = p.currency {
                v.push(("currency", c.clone()));
            }
            v
        }).unwrap_or_default();
        self.client
            .get_with_params_auth("/v1/fiat-payouts/crypto-currencies", &query, jwt_token)
            .await
    }

    /// Get payment methods for fiat payout
    pub async fn get_fiat_payouts_payment_methods(
        &self,
        params: Option<&FiatPayoutsCryptoParams>,
        jwt_token: Option<&str>,
    ) -> Result<FiatPayoutPaymentMethodsResponse, NowPaymentsError> {
        let query = params.map(|p| {
            let mut v: Vec<(&str, String)> = vec![];
            if let Some(ref pr) = p.provider {
                v.push(("provider", pr.clone()));
            }
            if let Some(ref c) = p.currency {
                v.push(("currency", c.clone()));
            }
            v
        }).unwrap_or_default();
        self.client
            .get_with_params_auth("/v1/fiat-payouts/payment-methods", &query, jwt_token)
            .await
    }

    /// List fiat payouts
    pub async fn get_fiat_payouts(
        &self,
        params: Option<&GetFiatPayoutsParams>,
        jwt_token: Option<&str>,
    ) -> Result<FiatPayoutsResponse, NowPaymentsError> {
        let query = params.map(|p| {
            let mut v: Vec<(&str, String)> = vec![];
            if let Some(ref id) = p.id {
                v.push(("id", id.clone()));
            }
            if let Some(ref pr) = p.provider {
                v.push(("provider", pr.clone()));
            }
            if let Some(ref rid) = p.request_id {
                v.push(("requestId", rid.clone()));
            }
            if let Some(ref fc) = p.fiat_currency {
                v.push(("fiatCurrency", fc.clone()));
            }
            if let Some(ref cc) = p.crypto_currency {
                v.push(("cryptoCurrency", cc.clone()));
            }
            if let Some(ref s) = p.status {
                v.push(("status", s.clone()));
            }
            if let Some(ref f) = p.filter {
                v.push(("filter", f.clone()));
            }
            if let Some(ref pid) = p.provider_payout_id {
                v.push(("providerPayoutId", pid.clone()));
            }
            if let Some(l) = p.limit {
                v.push(("limit", l.to_string()));
            }
            if let Some(pg) = p.page {
                v.push(("page", pg.to_string()));
            }
            if let Some(ref ob) = p.order_by {
                v.push(("orderBy", ob.clone()));
            }
            if let Some(ref sb) = p.sort_by {
                v.push(("sortBy", sb.clone()));
            }
            if let Some(ref df) = p.date_from {
                v.push(("dateFrom", df.clone()));
            }
            if let Some(ref dt) = p.date_to {
                v.push(("dateTo", dt.clone()));
            }
            v
        }).unwrap_or_default();
        self.client
            .get_with_params_auth("/v1/fiat-payouts", &query, jwt_token)
            .await
    }

    /// Get custody balance
    pub async fn get_balance(&self, jwt_token: Option<&str>) -> Result<Value, NowPaymentsError> {
        self.client.get_auth("/v1/balance", jwt_token).await
    }

    // --- Custody ---

    /// Create new sub-partner
    pub async fn create_sub_partner(
        &self,
        name: &str,
        jwt_token: &str,
    ) -> Result<CreateSubPartnerResponse, NowPaymentsError> {
        if jwt_token.trim().is_empty() {
            return Err(NowPaymentsError::new(
                "JWT token is required for createSubPartner. Call get_auth_token first.",
                None,
                None,
                None,
            ));
        }
        let body = serde_json::json!({ "name": name });
        self.client
            .post_auth("/v1/sub-partner/balance", body, jwt_token)
            .await
    }

    /// Create sub-partner payment (deposit)
    pub async fn create_sub_partner_payment(
        &self,
        params: &CreateSubPartnerPaymentParams,
        jwt_token: &str,
    ) -> Result<SubPartnerPaymentResponse, NowPaymentsError> {
        if jwt_token.trim().is_empty() {
            return Err(NowPaymentsError::new(
                "JWT token is required for createSubPartnerPayment. Call get_auth_token first.",
                None,
                None,
                None,
            ));
        }
        let body = serde_json::to_value(params).map_err(|e| {
            NowPaymentsError::new(&e.to_string(), None, None, None)
        })?;
        self.client
            .post_auth("/v1/sub-partner/payment", body, jwt_token)
            .await
    }

    /// Create subscription
    pub async fn create_subscription(
        &self,
        params: &CreateSubscriptionParams,
        jwt_token: &str,
    ) -> Result<CreateSubscriptionResponse, NowPaymentsError> {
        if jwt_token.trim().is_empty() {
            return Err(NowPaymentsError::new(
                "JWT token is required for createSubscription. Call get_auth_token first.",
                None,
                None,
                None,
            ));
        }
        let body = serde_json::to_value(params).map_err(|e| {
            NowPaymentsError::new(&e.to_string(), None, None, None)
        })?;
        self.client
            .post_auth("/v1/subscriptions", body, jwt_token)
            .await
    }

    /// Create transfer between user accounts
    pub async fn create_transfer(
        &self,
        params: &CreateTransferParams,
        jwt_token: &str,
    ) -> Result<Value, NowPaymentsError> {
        if jwt_token.trim().is_empty() {
            return Err(NowPaymentsError::new(
                "JWT token is required for createTransfer. Call get_auth_token first.",
                None,
                None,
                None,
            ));
        }
        let body = serde_json::to_value(params).map_err(|e| {
            NowPaymentsError::new(&e.to_string(), None, None, None)
        })?;
        self.client
            .post_auth("/v1/sub-partner/transfer", body, jwt_token)
            .await
    }

    /// Write off from user to master account
    pub async fn write_off(
        &self,
        params: &WriteOffParams,
        jwt_token: &str,
    ) -> Result<Value, NowPaymentsError> {
        if jwt_token.trim().is_empty() {
            return Err(NowPaymentsError::new(
                "JWT token is required for writeOff. Call get_auth_token first.",
                None,
                None,
                None,
            ));
        }
        let body = serde_json::to_value(params).map_err(|e| {
            NowPaymentsError::new(&e.to_string(), None, None, None)
        })?;
        self.client
            .post_auth("/v1/sub-partner/write-off", body, jwt_token)
            .await
    }

    /// Deposit from master to user account
    pub async fn deposit(
        &self,
        params: &DepositParams,
        jwt_token: &str,
    ) -> Result<Value, NowPaymentsError> {
        if jwt_token.trim().is_empty() {
            return Err(NowPaymentsError::new(
                "JWT token is required for deposit. Call get_auth_token first.",
                None,
                None,
                None,
            ));
        }
        let body = serde_json::to_value(params).map_err(|e| {
            NowPaymentsError::new(&e.to_string(), None, None, None)
        })?;
        self.client
            .post_auth("/v1/sub-partner/deposit", body, jwt_token)
            .await
    }

    // --- Conversions ---

    /// Create conversion within custody account
    pub async fn create_conversion(
        &self,
        params: &CreateConversionParams,
        jwt_token: &str,
    ) -> Result<Value, NowPaymentsError> {
        if jwt_token.trim().is_empty() {
            return Err(NowPaymentsError::new(
                "JWT token is required for createConversion. Call get_auth_token first.",
                None,
                None,
                None,
            ));
        }
        let body = serde_json::to_value(params).map_err(|e| {
            NowPaymentsError::new(&e.to_string(), None, None, None)
        })?;
        self.client
            .post_auth("/v1/conversion", body, jwt_token)
            .await
    }

    /// Get conversion status
    pub async fn get_conversion_status(
        &self,
        conversion_id: &str,
        jwt_token: &str,
    ) -> Result<Value, NowPaymentsError> {
        if jwt_token.trim().is_empty() {
            return Err(NowPaymentsError::new(
                "JWT token is required for getConversionStatus. Call get_auth_token first.",
                None,
                None,
                None,
            ));
        }
        self.client
            .get_auth(&format!("/v1/conversion/{}", conversion_id), Some(jwt_token))
            .await
    }

    /// List conversions
    pub async fn get_conversions(
        &self,
        params: Option<&GetConversionsParams>,
        jwt_token: Option<&str>,
    ) -> Result<Value, NowPaymentsError> {
        let query = params.map(|p| {
            let mut v: Vec<(&str, String)> = vec![];
            if let Some(ref id) = p.id {
                crate::query::push_json_filter_query(&mut v, "id", id);
            }
            if let Some(ref s) = p.status {
                crate::query::push_json_filter_query(&mut v, "status", s);
            }
            if let Some(ref fc) = p.from_currency {
                v.push(("from_currency", fc.clone()));
            }
            if let Some(ref tc) = p.to_currency {
                v.push(("to_currency", tc.clone()));
            }
            if let Some(ref cf) = p.created_at_from {
                v.push(("created_at_from", cf.clone()));
            }
            if let Some(ref ct) = p.created_at_to {
                v.push(("created_at_to", ct.clone()));
            }
            if let Some(l) = p.limit {
                v.push(("limit", l.to_string()));
            }
            if let Some(o) = p.offset {
                v.push(("offset", o.to_string()));
            }
            if let Some(ref ord) = p.order {
                v.push(("order", ord.clone()));
            }
            v
        }).unwrap_or_default();
        self.client
            .get_with_params_auth("/v1/conversion", &query, jwt_token)
            .await
    }

    /// Verify IPN webhook signature (uses ipn_secret from config)
    pub fn verify_ipn(&self, payload: &str, signature: &str) -> Result<bool, NowPaymentsError> {
        let secret = self.config.ipn_secret.as_ref().ok_or_else(|| {
            NowPaymentsError::new(
                "IPN secret not configured. Pass ipn_secret in config or use verify_ipn_signature() with explicit secret.",
                None,
                None,
                None,
            )
        })?;
        Ok(verify_ipn_signature(payload, signature, secret))
    }
}
