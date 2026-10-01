//! Human-friendly helpers for payment status and display

use crate::types::{Payment, PaymentStatus};
use std::collections::HashMap;

/// Parse payment status from API string
pub fn parse_payment_status(s: &str) -> Option<PaymentStatus> {
    match s {
        "waiting" => Some(PaymentStatus::Waiting),
        "confirming" => Some(PaymentStatus::Confirming),
        "confirmed" => Some(PaymentStatus::Confirmed),
        "spending" => Some(PaymentStatus::Spending),
        "sending" => Some(PaymentStatus::Sending),
        "partially_paid" => Some(PaymentStatus::PartiallyPaid),
        "finished" => Some(PaymentStatus::Finished),
        "failed" => Some(PaymentStatus::Failed),
        "refunded" => Some(PaymentStatus::Refunded),
        "expired" => Some(PaymentStatus::Expired),
        _ => None,
    }
}

/// User-friendly labels for payment statuses
pub fn payment_status_labels() -> HashMap<PaymentStatus, &'static str> {
    let mut m = HashMap::new();
    m.insert(PaymentStatus::Waiting, "Awaiting payment");
    m.insert(PaymentStatus::Confirming, "Confirming");
    m.insert(PaymentStatus::Confirmed, "Confirmed");
    m.insert(PaymentStatus::Spending, "Processing");
    m.insert(PaymentStatus::Sending, "Sending to wallet");
    m.insert(PaymentStatus::PartiallyPaid, "Partially paid");
    m.insert(PaymentStatus::Finished, "Completed");
    m.insert(PaymentStatus::Failed, "Failed");
    m.insert(PaymentStatus::Refunded, "Refunded");
    m.insert(PaymentStatus::Expired, "Expired");
    m
}

/// Check if payment is complete (success or terminal state)
pub fn is_payment_complete(status: PaymentStatus) -> bool {
    matches!(
        status,
        PaymentStatus::Finished
            | PaymentStatus::Failed
            | PaymentStatus::Refunded
            | PaymentStatus::Expired
    )
}

/// Check if payment is complete by status string (from API)
pub fn is_payment_complete_str(status: &str) -> bool {
    status.parse::<PaymentStatus>().map_or(false, is_payment_complete)
}

/// Check if payment is still pending (customer should pay)
pub fn is_payment_pending(status: PaymentStatus) -> bool {
    matches!(
        status,
        PaymentStatus::Waiting
            | PaymentStatus::Confirming
            | PaymentStatus::Confirmed
            | PaymentStatus::Spending
            | PaymentStatus::Sending
            | PaymentStatus::PartiallyPaid
    )
}

/// Check if payment is pending by status string (from API)
pub fn is_payment_pending_str(status: &str) -> bool {
    status.parse::<PaymentStatus>().map_or(false, is_payment_pending)
}

/// Get human-readable status label
pub fn get_status_label(status: PaymentStatus) -> String {
    let labels = payment_status_labels();
    labels
        .get(&status)
        .map(|s| s.to_string())
        .unwrap_or_else(|| format!("{:?}", status))
}

/// Get human-readable status label from string (from API)
pub fn get_status_label_str(status: &str) -> String {
    status
        .parse::<PaymentStatus>()
        .map(get_status_label)
        .unwrap_or_else(|_| status.to_string())
}

/// Build a short summary for displaying to users
/// e.g. "Awaiting payment: 0.001234 BTC → bc1q..."
pub fn get_payment_summary(payment: &Payment) -> String {
    let label = get_status_label_str(&payment.payment_status);
    let curr = payment.pay_currency.to_uppercase();
    let address = if payment.pay_address.trim().is_empty() {
        "…"
    } else {
        payment.pay_address.as_str()
    };
    format!(
        "{}: {} {} → {}",
        label,
        payment.pay_amount,
        curr,
        address
    )
}
