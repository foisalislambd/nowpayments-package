# nowpayments-ruby

<p align="center">
  <img src="https://img.shields.io/gem/v/nowpayments?color=6366f1&style=for-the-badge&logo=ruby" alt="gem version" />
  <img src="https://img.shields.io/badge/license-MIT-22c55e?style=for-the-badge" alt="license" />
  <img src="https://img.shields.io/badge/Ruby-2.7+-cc342d?style=for-the-badge&logo=ruby" alt="Ruby" />
</p>

<h1 align="center">nowpayments-ruby</h1>
<p align="center">
  <strong>Full-featured Ruby SDK for NOWPayments</strong><br/>
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
| **Helpers** | `payment_complete?`, `status_label`, etc. |

---

## 🚀 Quick Start

```bash
gem install nowpayments-ruby
```

Or add to your Gemfile:

```ruby
gem "nowpayments-ruby"
```

```ruby
require "nowpayments"

np = NowPayments::Client.new(
  api_key: ENV["NOWPAYMENTS_API_KEY"],
  sandbox: true  # false for production
)

# Create payment
payment = np.create_payment(
  price_amount: 29.99,
  price_currency: "usd",
  pay_currency: "btc",
  order_id: "order-123"
)

puts "Pay #{payment["pay_amount"]} #{(payment["pay_currency"] || "").to_s.upcase} → #{payment["pay_address"]}"
```

---

## ⚙️ Config

| Option | Required | Default | Description |
|--------|----------|---------|-------------|
| `api_key` | Yes | — | From [Dashboard](https://account.nowpayments.io) |
| `sandbox` | No | `false` | Use sandbox API |
| `timeout` | No | `30000` | Request timeout (ms) |
| `ipn_secret` | No | — | For webhook verification |
| `base_url` | No | — | Override API URL |

---

## 📁 Examples

All examples use the same setup. Set `NOWPAYMENTS_API_KEY` before running:

```bash
export NOWPAYMENTS_API_KEY=your_api_key
```

| File | Description | Run |
|------|-------------|-----|
| `01_create_payment.rb` | Create payment, show address to customer | `ruby examples/01_create_payment.rb` |
| `02_check_status.rb` | Check payment status + helper labels | `ruby examples/02_check_status.rb PAYMENT_ID` |
| `03_list_payments.rb` | List payments with filters | `ruby examples/03_list_payments.rb` |
| `04_create_invoice.rb` | Create invoice (redirect flow) | `ruby examples/04_create_invoice.rb` |
| `05_estimate_and_min_amount.rb` | Price estimate + minimum amount | `ruby examples/05_estimate_and_min_amount.rb` |
| `06_get_currencies.rb` | Get available currencies | `ruby examples/06_get_currencies.rb` |
| `07_payout_flow.rb` | Payout: validate → create → verify | `ruby examples/07_payout_flow.rb` |
| `08_subscription.rb` | Subscription plans + create subscription | `ruby examples/08_subscription.rb` |
| `09_ipn_webhook.rb` | IPN signature verification demo | `ruby examples/09_ipn_webhook.rb` |
| `10_custody_and_balance.rb` | Balance, sub-partners, deposit payment | `ruby examples/10_custody_and_balance.rb` |
| `11_conversions.rb` | Create conversion (custody) | `ruby examples/11_conversions.rb` |

### Example: Create Payment

```ruby
# examples/01_create_payment.rb
require "nowpayments"

np = NowPayments::Client.new(api_key: ENV["NOWPAYMENTS_API_KEY"], sandbox: true)

payment = np.create_payment(
  price_amount: 29.99,
  price_currency: "usd",
  pay_currency: "btc",
  order_id: "order-#{Time.now.to_i}",
  order_description: "Premium Plan",
  ipn_callback_url: "https://yoursite.com/webhook"
)

puts "Payment created: #{payment["payment_id"]}"
puts "Pay #{payment["pay_amount"]} #{payment["pay_currency"].to_s.upcase} to:"
puts payment["pay_address"]
```

### Example: Check Payment Status

```ruby
# examples/02_check_status.rb
payment_id = ARGV[0] || "PASTE_PAYMENT_ID_HERE"
payment = np.get_payment_status(payment_id)

puts "Status: #{NowPayments::Helpers.status_label(payment["payment_status"])}"
puts "Amount: #{payment["pay_amount"]} #{payment["pay_currency"]}"

if NowPayments::Helpers.payment_complete?(payment["payment_status"])
  puts "✅ Payment done! Fulfill the order."
else
  puts "⏳ Waiting for payment..."
end
```

### Example: List Payments

```ruby
# examples/03_list_payments.rb
result = np.get_payments(
  limit: 5,
  page: 0,
  sortBy: "created_at",
  orderBy: "desc",
  dateFrom: "2024-01-01",
  dateTo: "2024-12-31"
)

puts "Total: #{result["total"]}"
puts "Page: #{result["page"] + 1} of #{result["pagesCount"]}"
result["data"].each_with_index do |p, i|
  puts "#{i + 1}. #{p["payment_id"]} | #{p["payment_status"]} | #{p["price_amount"]} #{p["price_currency"]}"
end
```

### Example: Create Invoice

```ruby
# examples/04_create_invoice.rb
invoice = np.create_invoice(
  price_amount: 49.99,
  price_currency: "usd",
  pay_currency: "btc",
  order_id: "inv-#{Time.now.to_i}",
  order_description: "Premium subscription",
  success_url: "https://yoursite.com/success",
  cancel_url: "https://yoursite.com/cancel"
)

puts "Redirect customer to: #{invoice["invoice_url"]}"
```

### Example: Estimate & Min Amount

```ruby
# examples/05_estimate_and_min_amount.rb
estimate = np.get_estimate_price(
  amount: 100,
  currency_from: "usd",
  currency_to: "btc"
)
puts "100 USD ≈ #{estimate["estimated_amount"]} BTC"

min = np.get_min_amount(
  currency_from: "usd",
  currency_to: "btc",
  fiat_equivalent: "usd"
)
puts "Min amount: #{min["min_amount"]} BTC (≈ #{min["fiat_equivalent"]} USD)"
```

### Example: Get Currencies

```ruby
# examples/06_get_currencies.rb
currencies = np.get_currencies["currencies"]
puts "Supported: #{currencies.first(15).join(", ")}..."
puts "Total: #{currencies.length}"

btc_info = np.get_currency("btc")
puts "BTC info: #{btc_info}"

full = np.get_full_currencies["currencies"]
btc_full = full.find { |c| c["code"]&.downcase == "btc" }
puts "BTC full: #{btc_full}"
```

### Example: Payout Flow (JWT required)

```ruby
# examples/07_payout_flow.rb
# Env: NOWPAYMENTS_API_KEY, EMAIL, PASSWORD, PAYOUT_ADDRESS, VERIFICATION_CODE?

# 1. Validate address
np.validate_payout_address(address: payout_address, currency: "btc")

# 2. Get JWT
auth = np.get_auth_token(ENV["EMAIL"], ENV["PASSWORD"])
token = auth["token"]

# 3. Create payout
payout = np.create_payout(
  {
    withdrawals: [{ address: payout_address, currency: "btc", amount: 0.0001 }],
    ipn_callback_url: "https://yoursite.com/payout-webhook"
  },
  token
)

# 4. Verify (2FA code from email)
np.verify_payout(payout["id"], ENV["VERIFICATION_CODE"], token)
```

### Example: Subscriptions

```ruby
# examples/08_subscription.rb
plans = np.get_subscription_plans
puts "Plans: #{plans["result"]&.length || 0}"

plan = plans["result"]&.first
if plan
  auth = np.get_auth_token(ENV["EMAIL"], ENV["PASSWORD"])
  sub = np.create_subscription(
    { subscription_plan_id: plan["id"], email: "customer@example.com" },
    auth["token"]
  )
  puts "Subscription: #{sub["result"]}"
end
```

### Example: IPN Webhook

```ruby
# examples/09_ipn_webhook.rb
# In Sinatra/Rails: verify before processing

np = NowPayments::Client.new(api_key: "...", ipn_secret: ENV["IPN_SECRET"])

# In your webhook handler:
# if np.verify_ipn(request.body.read, request.env["HTTP_X_NOWPAYMENTS_SIG"])
#   payload = JSON.parse(request.body.read)
#   if NowPayments::Helpers.payment_complete?(payload["payment_status"])
#     # Fulfill order
#   end
# end

# Standalone verification
valid = NowPayments::IPN.verify_signature(payload, signature, ipn_secret)
NowPayments::IPN.create_signature(payload, ipn_secret)  # For testing
```

### Example: Custody & Balance

```ruby
# examples/10_custody_and_balance.rb
balance = np.get_balance
puts "Balance: #{balance}"

partners = np.get_sub_partners
puts "Sub-partners: #{partners}"

# Deposit with payment (JWT required)
auth = np.get_auth_token(ENV["EMAIL"], ENV["PASSWORD"])
result = np.create_sub_partner_payment(
  { currency: "trx", amount: 50, sub_partner_id: ENV["SUB_PARTNER_ID"] },
  auth["token"]
)
puts "Deposit: #{result["result"]["pay_address"]} #{result["result"]["pay_amount"]} TRX"
```

### Example: Conversions (JWT required)

```ruby
# examples/11_conversions.rb
auth = np.get_auth_token(ENV["EMAIL"], ENV["PASSWORD"])
token = auth["token"]

conv = np.create_conversion(
  { amount: 0.001, from_currency: "btc", to_currency: "usd" },
  token
)

if conv["deposit_id"]
  status = np.get_conversion_status(conv["deposit_id"], token)
  puts "Status: #{status}"
end
```

---

## 📖 API Reference

### Auth & Status

```ruby
np.get_status
auth = np.get_auth_token("your@email.com", "password")
token = auth["token"]
```

### Currencies

```ruby
np.get_currencies
np.get_currencies(true)  # fixed rate
np.get_full_currencies
np.get_merchant_coins
np.get_currency("btc")
```

### Payments

```ruby
payment = np.create_payment(
  price_amount: 29.99,
  price_currency: "usd",
  pay_currency: "btc",
  order_id: "order-123",
  ipn_callback_url: "https://yoursite.com/webhook",
  is_fixed_rate: true
)

np.get_payment_status(payment_id)
np.get_payments(limit: 10, page: 0, sortBy: "created_at", orderBy: "desc")
np.update_payment_estimate(payment_id)
```

### Estimate & Min Amount

```ruby
np.get_estimate_price(amount: 100, currency_from: "usd", currency_to: "btc")
np.get_min_amount(currency_from: "usd", currency_to: "btc", fiat_equivalent: "usd")
```

### Invoices

```ruby
invoice = np.create_invoice(
  price_amount: 49.99,
  price_currency: "usd",
  order_id: "inv-001",
  success_url: "https://yoursite.com/success",
  cancel_url: "https://yoursite.com/cancel"
)
# invoice["invoice_url"] → redirect customer

np.create_invoice_payment(iid: invoice_id, pay_currency: "btc")
```

### Payouts (JWT required)

```ruby
np.validate_payout_address(address: "0x...", currency: "eth")

batch = np.create_payout(
  {
    ipn_callback_url: "https://yoursite.com/payout-webhook",
    withdrawals: [
      { address: "TEmGw...", currency: "trx", amount: 200 },
      { address: "0x1EB...", currency: "eth", amount: 0.1 }
    ]
  },
  token
)

np.verify_payout(batch["id"], "123456", token)
np.cancel_payout(payout_id, token)
np.get_payout_status(payout_id, token)
np.get_payouts(batch_id: "...", limit: 10, page: 0)
```

### Fiat Payouts

```ruby
np.get_fiat_payouts_crypto_currencies({ provider: "transfi" }, token)
np.get_fiat_payouts_payment_methods({ provider: "transfi", currency: "usd" }, token)
np.get_fiat_payouts({ status: "FINISHED", limit: 10 }, token)
```

### Balance

```ruby
np.get_balance(token)
```

### Subscriptions

```ruby
np.get_subscription_plans(limit: 10, offset: 0)
np.get_subscription_plan(id)
np.update_subscription_plan(id, amount: 9.99, interval_day: "30")
np.create_subscription({ subscription_plan_id: 76215585, email: "user@example.com" }, token)
np.get_subscriptions(status: "PAID", limit: 10)
np.get_subscription(id)
np.delete_subscription(id, token)
```

### Custody / Sub-partners (JWT required)

```ruby
np.create_sub_partner("user-123", token)
np.create_sub_partner_payment({ currency: "trx", amount: 50, sub_partner_id: "1631380403" }, token)
np.get_sub_partners({ offset: 0, limit: 10 }, token)
np.get_sub_partner_balance(sub_partner_id)
np.create_transfer({ currency: "trx", amount: 0.3, from_id: 111, to_id: 222 }, token)
np.deposit({ currency: "trx", amount: 0.5, sub_partner_id: "111" }, token)
np.write_off({ currency: "trx", amount: 0.3, sub_partner_id: "111" }, token)
np.get_transfers({ status: "FINISHED", limit: 10 }, token)
np.get_transfer(id, token)
```

### Conversions (JWT required)

```ruby
np.create_conversion({ amount: 50, from_currency: "usdttrc20", to_currency: "usdterc20" }, token)
np.get_conversion_status(conversion_id, token)
np.get_conversions({ status: "FINISHED", limit: 10 }, token)
```

### IPN / Webhooks

```ruby
np = NowPayments::Client.new(api_key: "...", ipn_secret: "SECRET")

# In webhook handler (Sinatra/Rails)
if np.verify_ipn(request.body.read, request.env["HTTP_X_NOWPAYMENTS_SIG"])
  payload = JSON.parse(request.body.read)
  # Process payment
end

# Standalone
NowPayments.verify_ipn_signature(payload, signature, ipn_secret)
NowPayments.create_ipn_signature(payload, ipn_secret)  # For testing
NowPayments::IPN.verify_signature(payload, signature, ipn_secret)
NowPayments::IPN.create_signature(payload, ipn_secret)
```

### Helper functions

```ruby
NowPayments::Helpers.payment_complete?(payment["payment_status"])
NowPayments::Helpers.payment_pending?(payment["payment_status"])
NowPayments::Helpers.status_label(payment["payment_status"])
NowPayments::Helpers.payment_summary(payment)

# Constants
NowPayments::PAYMENT_STATUSES
NowPayments::PAYMENT_DONE_STATUSES
NowPayments::PAYMENT_PENDING_STATUSES
NowPayments::PAYMENT_STATUS_LABELS
```

### Error handling

```ruby
begin
  np.create_payment(...)
rescue NowPayments::NowPaymentsError => e
  puts e.message
  puts e.status_code
  puts e.code
  puts e.response
  puts e.to_s
end
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
