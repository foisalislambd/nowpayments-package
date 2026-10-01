package nowpayments

import (
	"fmt"
	"strings"
)

// PaymentStatusLabels maps status to human-readable label.
var PaymentStatusLabels = map[PaymentStatus]string{
	StatusWaiting:       "Awaiting payment",
	StatusConfirming:    "Confirming",
	StatusConfirmed:     "Confirmed",
	StatusSpending:      "Processing",
	StatusSending:       "Sending to wallet",
	StatusPartiallyPaid: "Partially paid",
	StatusFinished:      "Completed",
	StatusFailed:        "Failed",
	StatusRefunded:      "Refunded",
	StatusExpired:       "Expired",
}

// IsPaymentComplete returns true if payment is in a terminal state (success or failure).
func IsPaymentComplete(status PaymentStatus) bool {
	for _, s := range PaymentDoneStatuses {
		if s == status {
			return true
		}
	}
	return false
}

// IsPaymentPending returns true if customer should still pay.
func IsPaymentPending(status PaymentStatus) bool {
	for _, s := range PaymentPendingStatuses {
		if s == status {
			return true
		}
	}
	return false
}

// GetStatusLabel returns human-readable label for a status.
func GetStatusLabel(status PaymentStatus) string {
	if label, ok := PaymentStatusLabels[status]; ok {
		return label
	}
	return string(status)
}

// GetPaymentSummary builds a short summary for display, e.g. "Awaiting payment: 0.001234 BTC → bc1q..."
func GetPaymentSummary(payment *Payment) string {
	if payment == nil {
		return ""
	}
	label := GetStatusLabel(payment.PaymentStatus)
	curr := strings.ToUpper(payment.PayCurrency)
	addr := payment.PayAddress
	if addr == "" {
		addr = "…"
	}
	return fmt.Sprintf("%s: %g %s → %s", label, payment.PayAmount, curr, addr)
}
