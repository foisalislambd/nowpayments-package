//! NOWPayments Rust SDK
//!
//! Full-featured Rust client for the NOWPayments cryptocurrency payment API.
//!
//! # Example
//!
//! ```rust,no_run
//! use nowpayments_rust::{NowPayments, NowPaymentsConfig};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = NowPayments::new(NowPaymentsConfig {
//!         api_key: "your-api-key".to_string(),
//!         sandbox: Some(true),
//!         ..Default::default()
//!     })?;
//!
//!     let status = client.get_status().await?;
//!     println!("API status: {:?}", status);
//!
//!     let payment = client.create_payment(nowpayments_rust::CreatePaymentParams {
//!         price_amount: 29.99,
//!         price_currency: "usd".to_string(),
//!         pay_currency: "btc".to_string(),
//!         order_id: Some("order-123".to_string()),
//!         ..Default::default()
//!     }).await?;
//!     println!("Pay {} {} to {}", payment.pay_amount, payment.pay_currency, payment.pay_address);
//!
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod error;
pub mod helpers;
pub mod http;
pub mod ipn;
pub mod query;
pub mod types;

pub use client::NowPayments;
pub use error::NowPaymentsError;
pub use helpers::{
    get_payment_summary, get_status_label, get_status_label_str, is_payment_complete,
    is_payment_complete_str, is_payment_pending, is_payment_pending_str, parse_payment_status,
    payment_status_labels,
};
pub use ipn::{create_ipn_signature, verify_ipn_signature};
pub use types::*;
