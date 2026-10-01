/**
 * Example: List payments with filters
 * Run: npx tsx examples/03-list-payments.ts
 */
import { NowPayments } from '../src';

const np = new NowPayments({
  apiKey: process.env.NOWPAYMENTS_API_KEY!,
  sandbox: true,
});

async function main() {
  const result = await np.getPayments({
    limit: 5,
    page: 0,
    sortBy: 'created_at',
    orderBy: 'desc',
    dateFrom: '2024-01-01',
    dateTo: '2024-12-31',
  });

  console.log('Total:', result.total);
  console.log('Page:', result.page + 1, 'of', result.pagesCount);
  console.log('\nPayments:');
  result.data.forEach((p, i) => {
    console.log(`${i + 1}. ${p.payment_id} | ${p.payment_status} | ${p.price_amount} ${p.price_currency}`);
  });
}

main().catch(console.error);
