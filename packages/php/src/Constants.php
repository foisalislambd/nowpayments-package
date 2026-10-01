<?php

declare(strict_types=1);

namespace NowPayments;

/**
 * NOWPayments API constants.
 */
final class Constants
{
    /** All possible payment statuses */
    public const PAYMENT_STATUSES = [
        'waiting',
        'confirming',
        'confirmed',
        'spending',
        'sending',
        'partially_paid',
        'finished',
        'failed',
        'refunded',
        'expired',
    ];

    /** Statuses that mean payment is done (success or terminal) */
    public const PAYMENT_DONE_STATUSES = [
        'finished',
        'failed',
        'refunded',
        'expired',
    ];

    /** Statuses that mean customer should still pay */
    public const PAYMENT_PENDING_STATUSES = [
        'waiting',
        'confirming',
        'confirmed',
        'spending',
        'sending',
        'partially_paid',
    ];

    /** User-friendly labels for payment statuses */
    public const PAYMENT_STATUS_LABELS = [
        'waiting' => 'Awaiting payment',
        'confirming' => 'Confirming',
        'confirmed' => 'Confirmed',
        'spending' => 'Processing',
        'sending' => 'Sending to wallet',
        'partially_paid' => 'Partially paid',
        'finished' => 'Completed',
        'failed' => 'Failed',
        'refunded' => 'Refunded',
        'expired' => 'Expired',
    ];
}
