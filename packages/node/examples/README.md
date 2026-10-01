# Examples

Each example focuses on one feature. Run with:

```bash
export NOWPAYMENTS_API_KEY=your_key   # or set in .env
npx tsx examples/01-create-payment.ts
```

| File | Description |
|------|-------------|
| `01-create-payment.ts` | Create a crypto payment |
| `02-check-payment-status.ts` | Check status by payment ID |
| `03-list-payments.ts` | List payments with filters |
| `04-create-invoice.ts` | Create invoice (redirect URL) |
| `05-estimate-and-min-amount.ts` | Price estimate + min amount |
| `06-get-currencies.ts` | Supported currencies |
| `07-payout-flow.ts` | Validate → Create → Verify payout |
| `08-subscription.ts` | Subscription plans & create subscription |
| `09-ipn-webhook.ts` | IPN verification (webhook handler) |
| `10-custody-and-balance.ts` | Balance + sub-partners + deposit with payment |
| `11-conversions.ts` | Create conversion |
| `basic-usage.ts` | Combined quick demo |
