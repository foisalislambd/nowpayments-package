<div align="center">

# 💎 nowpayments-rust

**Full-featured Rust SDK for the [NOWPayments](https://nowpayments.io) cryptocurrency payment API**

[![Crates.io](https://img.shields.io/crates/v/nowpayments-rust.svg)](https://crates.io/crates/nowpayments-rust)
[![Documentation](https://docs.rs/nowpayments-rust/badge.svg)](https://docs.rs/nowpayments-rust)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust 1.70+](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)

*Accept Bitcoin, Ethereum, and 300+ cryptocurrencies in your Rust applications*

---

[Installation](#installation) • [Quick Start](#configuration) • [Examples](#examples) • [API Reference](#api-reference)

</div>

---

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
nowpayments-rust = "1.0"
tokio = { version = "1", features = ["full"] }
```

## Configuration

```rust
use nowpayments_rust::{NowPayments, NowPaymentsConfig};

let client = NowPayments::new(NowPaymentsConfig {
    api_key: std::env::var("NOWPAYMENTS_API_KEY").unwrap_or_else(|_| "your-api-key".into()),
    sandbox: Some(true),  // Use sandbox for testing
    base_url: None,       // Optional: override API URL
    timeout: Some(30000), // Request timeout in ms (default: 30000)
    ipn_secret: None,     // Optional: for IPN webhook verification
})?;
```

## Examples

### 1. Basic Usage – API Status & Create Payment

```rust
use nowpayments_rust::{NowPayments, NowPaymentsConfig, CreatePaymentParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = NowPayments::new(NowPaymentsConfig {
        api_key: std::env::var("NOWPAYMENTS_API_KEY").unwrap_or_default(),
        sandbox: Some(true),
        ..Default::default()
    })?;

    // Check API status
    let status = client.get_status().await?;
    println!("API status: {:?}", status);

    // Create a payment
    let payment = client.create_payment(CreatePaymentParams {
        price_amount: 29.99,
        price_currency: "usd".to_string(),
        pay_currency: "btc".to_string(),
        order_id: Some("order-123".to_string()),
        order_description: Some("Premium plan".to_string()),
        ..Default::default()
    }).await?;

    println!("Pay {} {} to {}", payment.pay_amount, payment.pay_currency, payment.pay_address);
    Ok(())
}
```

### 2. Check Payment Status

```rust
use nowpayments_rust::{NowPayments, NowPaymentsConfig, get_payment_summary};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = NowPayments::new(NowPaymentsConfig {
        api_key: std::env::var("NOWPAYMENTS_API_KEY").unwrap_or_default(),
        sandbox: Some(true),
        ..Default::default()
    })?;

    let payment_id = "12345678"; // Replace with actual payment ID
    let payment = client.get_payment_status(payment_id).await?;

    println!("Status: {}", payment.payment_status);
    println!("Summary: {}", get_payment_summary(&payment));
    Ok(())
}
```

### 3. List Payments

```rust
use nowpayments_rust::{NowPayments, NowPaymentsConfig, ListPaymentsParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = NowPayments::new(NowPaymentsConfig {
        api_key: std::env::var("NOWPAYMENTS_API_KEY").unwrap_or_default(),
        sandbox: Some(true),
        ..Default::default()
    })?;

    let list = client.get_payments(
        Some(&ListPaymentsParams {
            limit: Some(10),
            page: Some(1),
            sort_by: Some("created_at".into()),
            order_by: Some("desc".into()),
            ..Default::default()
        }),
        None,
    )
    .await?;

    println!("Total: {}, Page: {}/{}", list.total, list.page, list.pages_count);
    for p in &list.data {
        println!("  {} - {} - {}", p.payment_id, p.payment_status, p.pay_amount);
    }
    Ok(())
}
```

### 4. Create Invoice

```rust
use nowpayments_rust::{NowPayments, NowPaymentsConfig, CreateInvoiceParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = NowPayments::new(NowPaymentsConfig {
        api_key: std::env::var("NOWPAYMENTS_API_KEY").unwrap_or_default(),
        sandbox: Some(true),
        ..Default::default()
    })?;

    let invoice = client.create_invoice(CreateInvoiceParams {
        price_amount: 49.99,
        price_currency: "usd".to_string(),
        pay_currency: Some("btc".into()),
        order_id: Some("inv-001".into()),
        success_url: Some("https://yoursite.com/success".into()),
        cancel_url: Some("https://yoursite.com/cancel".into()),
        ..Default::default()
    }).await?;

    println!("Invoice URL: {}", invoice.invoice_url);
    Ok(())
}
```

### 5. Estimate Price & Min Amount

```rust
use nowpayments_rust::{NowPayments, NowPaymentsConfig, EstimateParams, MinAmountParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = NowPayments::new(NowPaymentsConfig {
        api_key: std::env::var("NOWPAYMENTS_API_KEY").unwrap_or_default(),
        sandbox: Some(true),
        ..Default::default()
    })?;

    // Get estimated crypto amount for 100 USD
    let estimate = client.get_estimate_price(&EstimateParams {
        amount: 100.0,
        currency_from: "usd".into(),
        currency_to: "btc".into(),
    }).await?;
    println!("100 USD ≈ {} BTC", estimate.estimated_amount);

    // Get minimum payment amount
    let min = client.get_min_amount(&MinAmountParams {
        currency_from: "usd".into(),
        currency_to: "btc".into(),
        ..Default::default()
    }).await?;
    println!("Min amount: {} {}", min.min_amount, min.currency_to);
    Ok(())
}
```

### 6. Get Currencies

```rust
use nowpayments_rust::{NowPayments, NowPaymentsConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = NowPayments::new(NowPaymentsConfig {
        api_key: std::env::var("NOWPAYMENTS_API_KEY").unwrap_or_default(),
        sandbox: Some(true),
        ..Default::default()
    })?;

    // Available currencies
    let currencies = client.get_currencies(None).await?;
    println!("Currencies: {:?}", currencies.currencies);

    // Full currency details
    let full = client.get_full_currencies().await?;
    for c in &full.currencies {
        println!("{} - {} (enabled: {})", c.code, c.name, c.enable);
    }

    // Merchant coins (from dashboard settings)
    let merchant = client.get_merchant_coins(Some(true)).await?;
    println!("Merchant coins: {:?}", merchant.currencies);
    Ok(())
}
```

### 7. Payout Flow (Validate → Create → Verify)

```rust
use nowpayments_rust::{NowPayments, NowPaymentsConfig, CreatePayoutParams, PayoutWithdrawal, ValidateAddressParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = NowPayments::new(NowPaymentsConfig {
        api_key: std::env::var("NOWPAYMENTS_API_KEY").unwrap_or_default(),
        sandbox: Some(true),
        ..Default::default()
    })?;

    let email = std::env::var("NOWPAYMENTS_EMAIL").unwrap_or_default();
    let password = std::env::var("NOWPAYMENTS_PASSWORD").unwrap_or_default();
    let token = client.get_auth_token(&email, &password).await?.token;

    // Validate address first
    client.validate_payout_address(&ValidateAddressParams {
        address: "bc1q...".to_string(),
        currency: "btc".into(),
        extra_id: None,
    }).await?;

    // Create payout
    let payout = client.create_payout(&CreatePayoutParams {
        withdrawals: vec![PayoutWithdrawal {
            address: "bc1q...".to_string(),
            currency: "btc".into(),
            amount: 0.001,
            ..Default::default()
        }],
        ..Default::default()
    }, &token).await?;

    println!("Payout batch ID: {}", payout.id);

    // Verify with 2FA code (if required)
    // let _ = client.verify_payout(&payout.withdrawals[0].id, "123456", &token).await?;
    Ok(())
}
```

### 8. Subscriptions

```rust
use nowpayments_rust::{NowPayments, NowPaymentsConfig, CreateSubscriptionParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = NowPayments::new(NowPaymentsConfig {
        api_key: std::env::var("NOWPAYMENTS_API_KEY").unwrap_or_default(),
        sandbox: Some(true),
        ..Default::default()
    })?;

    let token = client.get_auth_token(
        &std::env::var("NOWPAYMENTS_EMAIL").unwrap_or_default(),
        &std::env::var("NOWPAYMENTS_PASSWORD").unwrap_or_default(),
    ).await?.token;

    // List subscription plans
    let plans = client.get_subscription_plans(None).await?;
    for p in &plans.result {
        println!("Plan {}: {} {} every {} days", p.id, p.amount, p.currency, p.interval_day);
    }

    // Create subscription (email or sub_partner_id)
    let sub = client.create_subscription(&CreateSubscriptionParams {
        subscription_plan_id: "plan_id".into(),
        email: Some("customer@example.com".into()),
        sub_partner_id: None,
    }, &token).await?;
    println!("Subscription created: {:?}", sub.result);
    Ok(())
}
```

### 9. IPN Webhook Verification

```rust
use nowpayments_rust::{NowPayments, NowPaymentsConfig, verify_ipn_signature, create_ipn_signature};

// In your web server handler (e.g. Axum, Actix):
fn handle_ipn_webhook(body: &str, signature: &str, ipn_secret: &str) -> bool {
    verify_ipn_signature(body, signature, ipn_secret)
}

// Or with client (uses ipn_secret from config):
// client.verify_ipn(&body, &signature)

// Create signature for testing
fn main() {
    let payload = serde_json::json!({"payment_id": 123, "status": "finished"});
    let sig = create_ipn_signature(&payload, "your-ipn-secret");
    println!("Test signature: {:?}", sig);
}
```

### 10. Custody & Balance

```rust
use nowpayments_rust::{NowPayments, NowPaymentsConfig, CreateSubPartnerPaymentParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = NowPayments::new(NowPaymentsConfig {
        api_key: std::env::var("NOWPAYMENTS_API_KEY").unwrap_or_default(),
        sandbox: Some(true),
        ..Default::default()
    })?;

    let token = client.get_auth_token(
        &std::env::var("NOWPAYMENTS_EMAIL").unwrap_or_default(),
        &std::env::var("NOWPAYMENTS_PASSWORD").unwrap_or_default(),
    ).await?.token;

    // Get balance
    let balance: serde_json::Value = client.get_balance(Some(&token)).await?;
    println!("Balance: {:?}", balance);

    // Create sub-partner (user account)
    let sp = client.create_sub_partner("user-123", &token).await?;
    println!("Sub-partner: {:?}", sp.result);

    // Deposit with payment (customer pays to top up sub-partner)
    let dep = client.create_sub_partner_payment(&CreateSubPartnerPaymentParams {
        currency: "btc".into(),
        amount: 0.001,
        sub_partner_id: sp.result.id.clone(),
        ..Default::default()
    }, &token).await?;
    println!("Deposit payment: pay {} to {}", dep.result.pay_amount, dep.result.pay_address);
    Ok(())
}
```

### 11. Conversions

```rust
use nowpayments_rust::{NowPayments, NowPaymentsConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = NowPayments::new(NowPaymentsConfig {
        api_key: std::env::var("NOWPAYMENTS_API_KEY").unwrap_or_default(),
        sandbox: Some(true),
        ..Default::default()
    })?;

    let token = client.get_auth_token(
        &std::env::var("NOWPAYMENTS_EMAIL").unwrap_or_default(),
        &std::env::var("NOWPAYMENTS_PASSWORD").unwrap_or_default(),
    ).await?.token;

    let conversion: serde_json::Value = client.create_conversion(
        &serde_json::json!({
            "amount": 0.001,
            "from_currency": "btc",
            "to_currency": "eth"
        }),
        &token,
    ).await?;
    println!("Conversion: {:?}", conversion);
    Ok(())
}
```

### 12. Payment Helpers

```rust
use nowpayments_rust::{is_payment_complete_str, is_payment_pending_str, get_status_label_str};

fn check_payment(status: &str) {
    if is_payment_complete_str(status) {
        println!("Payment finished: {}", get_status_label_str(status));
    } else if is_payment_pending_str(status) {
        println!("Still pending: {}", get_status_label_str(status));
    }
}
```

## Running Examples

Set your API key and run:

```bash
export NOWPAYMENTS_API_KEY=your_api_key
cargo run --example create_payment
```

For payout/subscription/custody examples, also set:

```bash
export NOWPAYMENTS_EMAIL=your@email.com
export NOWPAYMENTS_PASSWORD=your_password
```

## API Reference

| Method | Description |
|--------|-------------|
| `get_status` | API health check |
| `get_currencies` | List available currencies |
| `get_full_currencies` | Full currency details |
| `get_merchant_coins` | Merchant-enabled coins |
| `get_currency` | Single currency info |
| `get_auth_token` | JWT for payouts/custody |
| `get_estimate_price` | Fiat → crypto estimate |
| `get_min_amount` | Min payment amount |
| `create_payment` | Create payment |
| `get_payment_status` | Get payment by ID |
| `get_payments` | List payments |
| `update_payment_estimate` | Refresh payment estimate |
| `create_invoice` | Create invoice |
| `create_invoice_payment` | Pay existing invoice |
| `get_subscriptions` | List subscriptions |
| `get_subscription` | Get subscription |
| `get_subscription_plans` | List plans |
| `create_subscription` | Create subscription |
| `create_payout` | Create payout (JWT) |
| `verify_payout` | Verify payout 2FA |
| `get_balance` | Custody balance |
| `create_sub_partner` | Create user (JWT) |
| `create_sub_partner_payment` | Deposit with payment |
| `create_conversion` | Convert in custody |
| `verify_ipn` | Verify IPN signature |

## License

MIT
