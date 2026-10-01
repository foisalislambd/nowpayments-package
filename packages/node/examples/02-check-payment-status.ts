/**
 * Example: Check payment status + show friendly message
 * Run: npx tsx examples/02-check-payment-status.ts
 */
import { NowPayments, getStatusLabel, isPaymentComplete } from '../src';

const np = new NowPayments({
  apiKey: process.env.NOWPAYMENTS_API_KEY!,
  sandbox: true,
});

const paymentId = process.argv[2] || 'PASTE_PAYMENT_ID_HERE';

async function main() {
  const payment = await np.getPaymentStatus(paymentId);

  console.log('Status:', getStatusLabel(payment.payment_status));
  console.log('Amount:', payment.pay_amount, payment.pay_currency);
  console.log('Paid:', payment.actually_paid ?? 0);

  if (isPaymentComplete(payment.payment_status)) {
    console.log('\n✅ Payment done! Fulfill the order.');
  } else {
    console.log('\n⏳ Waiting for payment...');
  }
}

main().catch(console.error);
