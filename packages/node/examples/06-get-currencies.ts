/**
 * Example: Get available currencies
 * Run: npx tsx examples/06-get-currencies.ts
 */
import { NowPayments } from '../src';

const np = new NowPayments({
  apiKey: process.env.NOWPAYMENTS_API_KEY!,
  sandbox: true,
});

async function main() {
  const { currencies } = await np.getCurrencies();
  console.log('Supported:', currencies.slice(0, 15).join(', '), '...');
  console.log('Total:', currencies.length);

  // Single currency info
  const btcInfo = await np.getCurrency('btc');
  console.log('\nBTC info:', btcInfo);

  // Full currency details (id, name, wallet_regex, network, etc.)
  const { currencies: full } = await np.getFullCurrencies();
  const btcFull = full.find((c) => c.code?.toLowerCase() === 'btc');
  console.log('\nBTC full:', btcFull);
}

main().catch(console.error);
