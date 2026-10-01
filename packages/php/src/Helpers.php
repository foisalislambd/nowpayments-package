<?php

declare(strict_types=1);

namespace NowPayments;

use NowPayments\Constants as C;

/**
 * Human-friendly helpers for payment status and display.
 */
class Helpers
{
    /**
     * Check if payment is complete (success or terminal state).
     */
    public static function isPaymentComplete(string $status): bool
    {
        return in_array($status, C::PAYMENT_DONE_STATUSES, true);
    }

    /**
     * Check if payment is still pending (customer should pay).
     */
    public static function isPaymentPending(string $status): bool
    {
        return in_array($status, C::PAYMENT_PENDING_STATUSES, true);
    }

    /**
     * Get human-readable status label.
     */
    public static function getStatusLabel(string $status): string
    {
        return C::PAYMENT_STATUS_LABELS[$status] ?? $status;
    }

    /**
     * Build a short summary for displaying to users.
     * e.g. "Awaiting payment: 0.001234 BTC → bc1q..."
     *
     * @param array<string, mixed> $payment
     */
    public static function getPaymentSummary(array $payment): string
    {
        $payAmount = $payment['pay_amount'] ?? 0;
        $payCurrency = strtoupper((string) ($payment['pay_currency'] ?? ''));
        $payAddress = $payment['pay_address'] ?? '…';
        $paymentStatus = (string) ($payment['payment_status'] ?? '');
        $label = C::PAYMENT_STATUS_LABELS[$paymentStatus] ?? $paymentStatus;
        return "{$label}: {$payAmount} {$payCurrency} → {$payAddress}";
    }
}
