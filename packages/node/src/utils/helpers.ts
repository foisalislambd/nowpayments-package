/**
 * Human-friendly helpers for payment status and display
 */

import type { Payment, PaymentStatus } from '../types';
import { PAYMENT_DONE_STATUSES, PAYMENT_PENDING_STATUSES } from '../types';

/** User-friendly labels for payment statuses */
export const PAYMENT_STATUS_LABELS: Record<PaymentStatus, string> = {
  waiting: 'Awaiting payment',
  confirming: 'Confirming',
  confirmed: 'Confirmed',
  spending: 'Processing',
  sending: 'Sending to wallet',
  partially_paid: 'Partially paid',
  finished: 'Completed',
  failed: 'Failed',
  refunded: 'Refunded',
  expired: 'Expired',
};

/**
 * Check if payment is complete (success or terminal state)
 */
export function isPaymentComplete(status: PaymentStatus): boolean {
  return PAYMENT_DONE_STATUSES.includes(status);
}

/**
 * Check if payment is still pending (customer should pay)
 */
export function isPaymentPending(status: PaymentStatus): boolean {
  return PAYMENT_PENDING_STATUSES.includes(status);
}

/**
 * Get human-readable status label
 */
export function getStatusLabel(status: PaymentStatus): string {
  return PAYMENT_STATUS_LABELS[status] ?? status;
}

/**
 * Build a short summary for displaying to users
 * e.g. "Awaiting payment: 0.001234 BTC → bc1q..."
 */
export function getPaymentSummary(payment: Payment): string {
  const { pay_amount, pay_currency, pay_address, payment_status } = payment;
  const label = PAYMENT_STATUS_LABELS[payment_status as PaymentStatus] ?? payment_status;
  const curr = (pay_currency || '').toUpperCase();
  return `${label}: ${pay_amount} ${curr} → ${pay_address || '…'}`;
}
