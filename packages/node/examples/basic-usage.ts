/**
 * Basic usage example for nowpayments-node
 *
 * Run: npx tsx examples/basic-usage.ts
 * Or set NOWPAYMENTS_API_KEY and run with: npx tsx examples/basic-usage.ts
 */

import {
  NowPayments,
  getStatusLabel,
  getPaymentSummary,
  isPaymentComplete,
} from '../src';

const apiKey = process.env.NOWPAYMENTS_API_KEY || 'YOUR_API_KEY';

async function main() {
  const np = new NowPayments({
    apiKey,
    sandbox: true, // Always use sandbox first!
  });

  // 1. Check if API is up
  const status = await np.getStatus();
  console.log('✓ API Status:', status);

  // 2. What cryptos can customers pay with?
  const { currencies } = await np.getCurrencies();
  console.log('✓ Supported:', currencies.slice(0, 8).join(', '), '...');

  // 3. How much is 100 USD in BTC?
  const estimate = await np.getEstimatePrice({
    amount: 100,
    currency_from: 'usd',
    currency_to: 'btc',
  });
  console.log(`✓ 100 USD ≈ ${estimate.estimated_amount} BTC`);

  // 4. Minimum payment for USD → BTC
  const minAmount = await np.getMinAmount({
    currency_from: 'usd',
    currency_to: 'btc',
  });
  console.log(`✓ Min: ${minAmount.min_amount} ${minAmount.currency_to.toUpperCase()}`);

  // 5. Create payment (uncomment when you have a real API key)
  /*
  const payment = await np.createPayment({
    price_amount: 10,
    price_currency: 'usd',
    pay_currency: 'btc',
    order_id: 'demo-order-1',
    order_description: 'Demo Product',
  });
  console.log('✓ Payment created:', getPaymentSummary(payment));

  // Check status later
  const updated = await np.getPaymentStatus(payment.payment_id);
  console.log('  Status:', getStatusLabel(updated.payment_status));
  if (isPaymentComplete(updated.payment_status)) {
    console.log('  Done! Fulfill the order.');
  }
  */
}

main().catch((err) => {
  console.error('Error:', err.message);
  if (err.message?.includes('API key')) {
    console.log('\nTip: Get your key at https://account-sandbox.nowpayments.io');
  }
  process.exit(1);
});
