/**
 * Example: Create a payment
 * Run: npx tsx examples/01-create-payment.ts
 */
import { NowPayments } from '../src';

const np = new NowPayments({
  apiKey: process.env.NOWPAYMENTS_API_KEY!,
  sandbox: true,
});

async function main() {
  const payment = await np.createPayment({
    price_amount: 29.99,
    price_currency: 'usd',
    pay_currency: 'btc',
    order_id: 'order-' + Date.now(),
    order_description: 'Premium Plan',
    ipn_callback_url: 'https://yoursite.com/webhook',
  });

  console.log('Payment created!');
  console.log('Pay:', payment.pay_amount, payment.pay_currency.toUpperCase());
  console.log('Address:', payment.pay_address);
  console.log('Payment ID:', payment.payment_id);
}

main().catch(console.error);
