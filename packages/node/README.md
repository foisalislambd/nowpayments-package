<p align="center">
  <img src="https://img.shields.io/npm/v/nowpayments-node?color=6366f1&style=for-the-badge&logo=npm" alt="npm version" />
  <img src="https://img.shields.io/badge/license-MIT-22c55e?style=for-the-badge" alt="license" />
  <img src="https://img.shields.io/badge/Node.js-16+-339933?style=for-the-badge&logo=node.js" alt="Node.js" />
  <img src="https://img.shields.io/badge/TypeScript-5+-3178c6?style=for-the-badge&logo=typescript" alt="TypeScript" />
</p>

<h1 align="center">nowpayments-node</h1>
<p align="center">
  <strong>Full-featured Node.js SDK for NOWPayments</strong><br/>
  Accept 300+ cryptocurrencies with auto-conversion to your wallet
</p>

<p align="center">
  <a href="#-quick-start">Quick Start</a> •
  <a href="#-features">Features</a> •
  <a href="#-examples">Examples</a> •
  <a href="#-api-reference">API</a> •
  <a href="#-links">Links</a>
</p>

---

## ✨ Features

| Feature | Support |
|---------|---------|
| **Payments** | Create, status, list, update estimate |
| **Invoices** | Create invoice + redirect flow |
| **Payouts** | Mass payout, verify 2FA, cancel scheduled |
| **Fiat Payouts** | Currencies, payment methods, list |
| **Subscriptions** | Plans, recurring payments, cancel |
| **Custody** | Sub-partners, transfers, deposit, write-off |
| **Conversions** | In-custody currency conversion |
| **IPN Webhooks** | HMAC signature verification |
| **Helpers** | `isPaymentComplete`, `getStatusLabel`, etc. |

---

## 🚀 Quick start

```bash
npm install nowpayments-node
```

```typescript
import { NowPayments } from 'nowpayments-node';

const np = new NowPayments({
  apiKey: process.env.NOWPAYMENTS_API_KEY!,
  sandbox: true,  // false for production
});

// Create payment
const payment = await np.createPayment({
  price_amount: 29.99,
  price_currency: 'usd',
  pay_currency: 'btc',
  order_id: 'order-123',
});

console.log(`Pay ${payment.pay_amount} BTC → ${payment.pay_address}`);
```

---

## ⚙️ Config

| Option | Required | Default | Description |
|--------|----------|---------|-------------|
| `apiKey` | Yes | — | From [Dashboard](https://account.nowpayments.io) |
| `sandbox` | No | `false` | Use sandbox API |
| `timeout` | No | `30000` | Request timeout (ms) |
| `ipnSecret` | No | — | For webhook verification |
| `baseUrl` | No | — | Override API URL |

---

## 📖 API Reference & Examples

> All examples use the same `np` instance. Initialize once with `const np = new NowPayments({ apiKey: '...', sandbox: true });` at the start.

---

### Auth & Status

#### `getStatus()` — Check if API is up
```typescript
const status = await np.getStatus();
console.log(status);  // { message: 'OK' }
```

#### `getAuthToken(email, password)` — Get JWT (required for payouts, custody)
```typescript
const { token } = await np.getAuthToken('your@email.com', 'password');
// Token expires in 5 min. Use it for createPayout, verifyPayout, createSubPartner, etc.
```

---

### Currencies

#### `getCurrencies(fixedRate?)` — List all available crypto
```typescript
const { currencies } = await np.getCurrencies();
// ['btc', 'eth', 'usdt', 'trx', ...]

// With fixed rate min/max:
const { currencies } = await np.getCurrencies(true);
```

#### `getFullCurrencies()` — Detailed info
```typescript
const { currencies } = await np.getFullCurrencies();
// currencies[0] → { id, code, name, wallet_regex, network, ... }
```

#### `getMerchantCoins(fixedRate?)` — Coins enabled in your dashboard
```typescript
const { currencies } = await np.getMerchantCoins();
```

#### `getCurrency(currency)` — Single currency details
```typescript
const info = await np.getCurrency('btc');
```

---

### Payments (main flow)

#### `createPayment(params)` — Create payment → show address to customer
```typescript
const payment = await np.createPayment({
  price_amount: 29.99,
  price_currency: 'usd',
  pay_currency: 'btc',
  order_id: 'order-12345',
  order_description: 'Monthly plan',
  ipn_callback_url: 'https://yoursite.com/webhook',  // optional
  is_fixed_rate: true,       // optional
  is_fee_paid_by_user: false,  // optional
});

// Show to customer:
console.log(`Pay ${payment.pay_amount} ${payment.pay_currency.toUpperCase()}`);
console.log(`To: ${payment.pay_address}`);
```

#### `getPaymentStatus(paymentId)` — Status check
```typescript
const payment = await np.getPaymentStatus(5524759814);
console.log(payment.payment_status);  // 'waiting' | 'finished' | 'expired' | ...
```

#### `getPayments(params?)` — List all payments
```typescript
const list = await np.getPayments({
  limit: 10,
  page: 0,
  sortBy: 'created_at',
  orderBy: 'desc',
  dateFrom: '2024-01-01',
  dateTo: '2024-12-31',
});
console.log(list.data, list.total, list.pagesCount);
```

#### `updatePaymentEstimate(paymentId)` — Refresh amount before expiry
```typescript
const result = await np.updatePaymentEstimate(paymentId);
// result.pay_amount, result.expiration_estimate_date
```

---

### Price & Minimum amount

#### `getEstimatePrice(params)` — Fiat → crypto conversion
```typescript
const estimate = await np.getEstimatePrice({
  amount: 100,
  currency_from: 'usd',
  currency_to: 'btc',
});
console.log(`100 USD ≈ ${estimate.estimated_amount} BTC`);
```

#### `getMinAmount(params)` — Minimum payment amount
```typescript
const min = await np.getMinAmount({
  currency_from: 'usd',
  currency_to: 'btc',
  fiat_equivalent: 'usd',   // optional
  is_fixed_rate: false,     // optional
  is_fee_paid_by_user: false,  // optional
});
console.log(min.min_amount, min.fiat_equivalent);
```

---

### Invoices

#### `createInvoice(params)` — Create invoice URL → redirect customer
```typescript
const invoice = await np.createInvoice({
  price_amount: 49.99,
  price_currency: 'usd',
  pay_currency: 'btc',  // optional
  order_id: 'inv-001',
  order_description: 'Premium',
  success_url: 'https://yoursite.com/success',
  cancel_url: 'https://yoursite.com/cancel',
  partially_paid_url: 'https://yoursite.com/partial',  // optional
  is_fixed_rate: true,
  is_fee_paid_by_user: false,
});
// invoice.invoice_url → redirect customer to this link
```

#### `createInvoicePayment(params)` — Payment for existing invoice
```typescript
const payment = await np.createInvoicePayment({
  iid: invoiceId,
  pay_currency: 'btc',
  purchase_id: 'purchase-123',
  order_description: 'Item',
  customer_email: 'user@example.com',
});
```

---

### Payouts (JWT required)

```typescript
const { token } = await np.getAuthToken(email, password);
```

#### `validatePayoutAddress(params)` — Validate address
```typescript
try {
  await np.validatePayoutAddress({ address: '0x...', currency: 'eth' });
  console.log('Valid');
} catch {
  console.log('Invalid');
}
```

#### `createPayout(params, jwtToken)` — Mass payout
```typescript
const batch = await np.createPayout({
  ipn_callback_url: 'https://yoursite.com/payout-webhook',
  withdrawals: [
    { address: 'TEmGw...', currency: 'trx', amount: 200 },
    { address: '0x1EB...', currency: 'eth', amount: 0.1 },
    // Scheduled payout: { ..., execute_at: '2024-12-31T10:00:00Z' }
  ],
}, token);
console.log(batch.id, batch.withdrawals);
```

#### `verifyPayout(payoutId, verificationCode, jwtToken)` — 2FA verify
```typescript
await np.verifyPayout(batch.id, '123456', token);
```

#### `cancelPayout(payoutId, jwtToken)` — Cancel scheduled payout
```typescript
// Use individual payout id, not batch id
await np.cancelPayout('5000000000', token);
```

#### `getPayoutStatus(payoutId, jwtToken?)`
```typescript
const status = await np.getPayoutStatus('5000000713', token);
```

#### `getPayouts(params?)`
```typescript
const payouts = await np.getPayouts({
  batch_id: '5000000713',
  status: 'finished',
  limit: 10,
  page: 0,
  order_by: 'dateCreated',
  order: 'desc',
});
```

---

### Fiat Payouts (JWT required)

#### `getFiatPayoutsCryptoCurrencies(params?, jwtToken?)`
```typescript
const { result } = await np.getFiatPayoutsCryptoCurrencies({ provider: 'transfi' }, token);
```

#### `getFiatPayoutsPaymentMethods(params?, jwtToken?)`
```typescript
const { result } = await np.getFiatPayoutsPaymentMethods(
  { provider: 'transfi', currency: 'usd' },
  token
);
```

#### `getFiatPayouts(params?, jwtToken?)`
```typescript
const { result } = await np.getFiatPayouts(
  { status: 'FINISHED', limit: 10, dateFrom: '2024-01-01' },
  token
);
// result.rows
```

---

### Balance

#### `getBalance(jwtToken?)`
```typescript
const balance = await np.getBalance(token);
// { eth: { amount: 0.5, pendingAmount: 0 }, trx: { ... } }
```

---

### Subscriptions (recurring)

#### `getSubscriptionPlans(params?)`
```typescript
const { result, count } = await np.getSubscriptionPlans({ limit: 10, offset: 0 });
```

#### `getSubscriptionPlan(id)`
```typescript
const { result } = await np.getSubscriptionPlan('76215585');
```

#### `updateSubscriptionPlan(id, updates)`
```typescript
await np.updateSubscriptionPlan('76215585', {
  amount: 9.99,
  interval_day: '30',
});
```

#### `createSubscription(params, jwtToken)` — Email or custody user
```typescript
// Email subscription:
const { result } = await np.createSubscription({
  subscription_plan_id: 76215585,
  email: 'customer@example.com',
}, token);

// Custody (sub-partner):
const { result } = await np.createSubscription({
  subscription_plan_id: 76215585,
  sub_partner_id: '111394288',
}, token);
```

#### `getSubscriptions(params?)`
```typescript
const { result, count } = await np.getSubscriptions({
  status: 'PAID',
  subscription_plan_id: '111394288',
  is_active: true,
  limit: 10,
  offset: 0,
});
```

#### `getSubscription(id)` / `deleteSubscription(id, jwtToken?)`
```typescript
const { result } = await np.getSubscription('1515573197');
await np.deleteSubscription('1515573197', token);
```

---

### Custody / Sub-partners (JWT required)

#### `createSubPartner(name, jwtToken)`
```typescript
const { result } = await np.createSubPartner('user-123', token);
// result.id, result.name
```

#### `createSubPartnerPayment(params, jwtToken)` — Top up sub-partner with crypto
```typescript
const { result } = await np.createSubPartnerPayment(
  { currency: 'trx', amount: 50, sub_partner_id: '1631380403', fixed_rate: false },
  token
);
// Show customer: Pay result.pay_amount TRX to result.pay_address
```

#### `getSubPartners(params?, jwtToken?)` / `getSubPartnerBalance(subPartnerId)`
```typescript
const users = await np.getSubPartners({ offset: 0, limit: 10, order: 'DESC' }, token);
const { result } = await np.getSubPartnerBalance('111394288');
// result.balances.usdtbsc.amount
```

#### `createTransfer(params, jwtToken)` — Transfer between users
```typescript
await np.createTransfer({
  currency: 'trx',
  amount: 0.3,
  from_id: 111394288,
  to_id: 5209391548,
}, token);
```

#### `deposit(params, jwtToken)` / `writeOff(params, jwtToken)`
```typescript
await np.deposit({
  currency: 'trx',
  amount: 0.5,
  sub_partner_id: '111394288',
}, token);

await np.writeOff({
  currency: 'trx',
  amount: 0.3,
  sub_partner_id: '111394288',
}, token);
```

#### `getTransfers(params?, jwtToken?)` / `getTransfer(id, jwtToken?)`
```typescript
const transfers = await np.getTransfers({ status: 'FINISHED', limit: 10 }, token);
const transfer = await np.getTransfer('327209161', token);
```

---

### Conversions (JWT required)

#### `createConversion(params, jwtToken)`
```typescript
await np.createConversion({
  amount: 50,
  from_currency: 'usdttrc20',
  to_currency: 'usdterc20',
}, token);
```

#### `getConversionStatus(conversionId, jwtToken)` / `getConversions(params?, jwtToken?)`
```typescript
const status = await np.getConversionStatus('1327866232', token);
const list = await np.getConversions({ status: 'FINISHED', limit: 10 }, token);
```

---

### IPN / Webhooks

```typescript
const np = new NowPayments({ apiKey: '...', ipnSecret: 'SECRET' });

// Express/Fastify handler এ:
if (np.verifyIpn(req.body, req.headers['x-nowpayments-sig'])) {
  const { payment_id, payment_status } = req.body;
  // process...
}

// Standalone (without instance):
import { verifyIpnSignature } from 'nowpayments-node';
verifyIpnSignature(req.body, sig, 'SECRET');
```

---

### Helper functions

```typescript
import {
  isPaymentComplete,   // finished, failed, refunded, expired
  isPaymentPending,   // waiting, confirming, ...
  getStatusLabel,     // "Awaiting payment" | "Completed" | ...
  getPaymentSummary,  // "Awaiting payment: 0.001 BTC → bc1q..."
  PAYMENT_STATUS_LABELS,
  PAYMENT_STATUSES,
} from 'nowpayments-node';

const payment = await np.getPaymentStatus(id);
console.log(getStatusLabel(payment.payment_status));
if (isPaymentComplete(payment.payment_status)) {
  // Fulfill order
}
```

---

### Error handling

```typescript
import { NowPaymentsError } from 'nowpayments-node';

try {
  await np.createPayment({ ... });
} catch (err) {
  if (err instanceof NowPaymentsError) {
    console.log(err.message);      // "Invalid api key"
    console.log(err.statusCode);   // 401
    console.log(err.toString());   // "Invalid api key (status: 401)"
  }
}
```

---

## 📁 Run examples (optional)

Clone the repo and run from the `examples/` folder:

| File | Description |
|------|-------------|
| `01-create-payment.ts` | Create payment |
| `02-check-payment-status.ts` | Check status |
| `03-list-payments.ts` | List payments |
| `04-create-invoice.ts` | Invoice |
| `05-estimate-and-min-amount.ts` | Price estimate |
| `06-get-currencies.ts` | Currencies |
| `07-payout-flow.ts` | Payout flow |
| `08-subscription.ts` | Subscriptions |
| `09-ipn-webhook.ts` | IPN verification |
| `10-custody-and-balance.ts` | Balance, custody |
| `11-conversions.ts` | Conversions |

```bash
export NOWPAYMENTS_API_KEY=your_key
npx tsx examples/01-create-payment.ts
```

---

## 🔗 Links

| Link | URL |
|------|-----|
| **API Docs** | [Postman](https://documenter.getpostman.com/view/7907941/2s93JusNJt) |
| **Sandbox** | [Postman Sandbox](https://documenter.getpostman.com/view/7907941/T1LSCRHC) |
| **Help** | [nowpayments.io/help](https://nowpayments.io/help/payments/api) |
| **Dashboard** | [account.nowpayments.io](https://account.nowpayments.io) |

---

## 📄 License

MIT © [Foisalislambd](https://github.com/Foisalislambd)
