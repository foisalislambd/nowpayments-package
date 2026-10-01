/**
 * Example: Get price estimate + minimum amount
 * Run: npx tsx examples/05-estimate-and-min-amount.ts
 */
import { NowPayments } from '../src';

const np = new NowPayments({
  apiKey: process.env.NOWPAYMENTS_API_KEY!,
  sandbox: true,
});

async function main() {
  // How much is 100 USD in BTC?
  const estimate = await np.getEstimatePrice({
    amount: 100,
    currency_from: 'usd',
    currency_to: 'btc',
  });
  console.log('100 USD ≈', estimate.estimated_amount, 'BTC');

  // Minimum payment for USD → BTC
  const min = await np.getMinAmount({
    currency_from: 'usd',
    currency_to: 'btc',
    fiat_equivalent: 'usd',
  });
  console.log('Min amount:', min.min_amount, 'BTC');
  console.log('(≈', min.fiat_equivalent, 'USD)');
}

main().catch(console.error);
