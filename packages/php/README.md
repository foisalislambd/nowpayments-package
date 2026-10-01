<p align="center">
  <img src="https://img.shields.io/packagist/v/foisalislambd/nowpayments-php?color=6366f1&style=for-the-badge" alt="Packagist version" />
  <img src="https://img.shields.io/badge/license-MIT-22c55e?style=for-the-badge" alt="license" />
  <img src="https://img.shields.io/badge/PHP-8.0+-777BB4?style=for-the-badge&logo=php" alt="PHP" />
  <img src="https://img.shields.io/badge/Composer-2+-885630?style=for-the-badge&logo=composer" alt="Composer" />
</p>

<h1 align="center">nowpayments-php</h1>

<p align="center">
  <strong>Full-featured PHP SDK for NOWPayments</strong><br/>
  Accept 300+ cryptocurrencies with auto-conversion to your wallet
</p>

<p align="center">
  <a href="#-quick-start">Quick Start</a> •
  <a href="#-installation">Installation</a> •
  <a href="#-configuration">Configuration</a> •
  <a href="#-features">Features</a> •
  <a href="#-api-reference--examples">API Reference</a> •
  <a href="#-run-examples">Examples</a> •
  <a href="#-links">Links</a>
</p>

---

## 📑 Table of Contents

- [Features](#-features)
- [Quick Start](#-quick-start)
- [Installation](#-installation)
- [Configuration](#️-configuration)
- [API Reference & Examples](#-api-reference--examples)
  - [Status & Auth](#-status--auth)
  - [Currencies](#-currencies)
  - [Payments](#-payments-main-flow)
  - [Price & Min Amount](#-price--minimum-amount)
  - [Invoices](#-invoices)
  - [Payouts](#-payouts-jwt-required)
  - [Fiat Payouts](#-fiat-payouts-jwt-required)
  - [Balance](#-balance)
  - [Subscriptions](#-subscriptions-recurring)
  - [Custody / Sub-partners](#-custody--sub-partners-jwt-required)
  - [Conversions](#-conversions-jwt-required)
  - [IPN / Webhooks](#-ipn--webhooks)
  - [Helpers](#-helper-functions)
  - [Error Handling](#️-error-handling)
- [Run Examples](#-run-examples)
- [Links](#-links)

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

## 🚀 Quick Start

```bash
composer require foisalislambd/nowpayments-php
```

```php
<?php
require 'vendor/autoload.php';

use NowPayments\NowPayments;

$np = new NowPayments([
    'apiKey' => getenv('NOWPAYMENTS_API_KEY'),
    'sandbox' => true,  // false for production
]);

// Create payment
$payment = $np->createPayment([
    'price_amount' => 29.99,
    'price_currency' => 'usd',
    'pay_currency' => 'btc',
    'order_id' => 'order-123',
]);

echo "Pay {$payment['pay_amount']} BTC → {$payment['pay_address']}\n";
```

---

## 📦 Installation

```bash
composer require foisalislambd/nowpayments-php
```

**Requirements:** PHP 8.0+, Guzzle HTTP 7.x

---

## ⚙️ Configuration

| Option | Required | Default | Description |
|--------|----------|---------|-------------|
| `apiKey` | Yes | — | From [Dashboard](https://account.nowpayments.io) |
| `sandbox` | No | `false` | Use sandbox API |
| `timeout` | No | `30` | Request timeout (seconds, or ms if > 100) |
| `ipnSecret` | No | — | For webhook verification |
| `baseUrl` | No | — | Override API URL |

```php
$np = new NowPayments([
    'apiKey'    => 'your_api_key',
    'sandbox'   => true,
    'timeout'   => 30,
    'ipnSecret' => 'your_ipn_secret',  // for verifyIpn()
    'baseUrl'   => null,               // optional override
]);
```

---

## 📖 API Reference & Examples

> All examples assume `$np` is initialized: `$np = new NowPayments(['apiKey' => '...', 'sandbox' => true]);`

---

### 🔌 Status & Auth

#### `getStatus()` — Check if API is up

```php
$status = $np->getStatus();
// ['message' => 'OK', ...]
```

#### `getAuthToken($email, $password)` — Get JWT (required for payouts, custody)

```php
$auth = $np->getAuthToken('your@email.com', 'password');
$token = $auth['token'];
// Token expires in 5 min. Use for createPayout, verifyPayout, createSubPartner, etc.
```

---

### 💱 Currencies

#### `getCurrencies($fixedRate?)` — List all available crypto

```php
$result = $np->getCurrencies();
// ['currencies' => ['btc', 'eth', 'usdt', 'trx', ...]]

// With fixed rate min/max:
$result = $np->getCurrencies(true);
```

#### `getFullCurrencies()` — Detailed currency info

```php
$result = $np->getFullCurrencies();
// ['currencies' => [['id' => 1, 'code' => 'btc', 'name' => 'Bitcoin', 'wallet_regex' => '...', ...], ...]]
```

#### `getMerchantCoins($fixedRate?)` — Coins enabled in your dashboard

```php
$result = $np->getMerchantCoins();
// ['currencies' => ['btc', 'eth', ...]]
```

#### `getCurrency($currency)` — Single currency details

```php
$info = $np->getCurrency('btc');
// ['id' => 1, 'code' => 'btc', 'name' => 'Bitcoin', ...]
```

---

### 💳 Payments (main flow)

#### `createPayment($params)` — Create payment → show address to customer

```php
$payment = $np->createPayment([
    'price_amount'      => 29.99,
    'price_currency'    => 'usd',
    'pay_currency'      => 'btc',
    'order_id'          => 'order-12345',
    'order_description' => 'Monthly plan',
    'ipn_callback_url'  => 'https://yoursite.com/webhook',  // optional
    'is_fixed_rate'     => true,       // optional
    'is_fee_paid_by_user' => false,   // optional
]);

// Show to customer:
echo "Pay {$payment['pay_amount']} " . strtoupper($payment['pay_currency']) . "\n";
echo "To: {$payment['pay_address']}\n";
```

#### `getPaymentStatus($paymentId)` — Status check

```php
$payment = $np->getPaymentStatus(5524759814);
echo $payment['payment_status'];  // 'waiting' | 'finished' | 'expired' | ...
```

#### `getPayments($params?)` — List all payments

```php
$list = $np->getPayments([
    'limit'   => 10,
    'page'    => 0,
    'sortBy'  => 'created_at',
    'orderBy' => 'desc',
    'dateFrom' => '2024-01-01',
    'dateTo'   => '2024-12-31',
]);
// $list['data'], $list['total'], $list['pagesCount']
```

#### `updatePaymentEstimate($paymentId)` — Refresh amount before expiry

```php
$result = $np->updatePaymentEstimate($paymentId);
// $result['pay_amount'], $result['expiration_estimate_date']
```

---

### 📊 Price & Minimum Amount

#### `getEstimatePrice($params)` — Fiat → crypto conversion

```php
$estimate = $np->getEstimatePrice([
    'amount'         => 100,
    'currency_from'  => 'usd',
    'currency_to'    => 'btc',
]);
echo "100 USD ≈ {$estimate['estimated_amount']} BTC\n";
```

#### `getMinAmount($params)` — Minimum payment amount

```php
$min = $np->getMinAmount([
    'currency_from'       => 'usd',
    'currency_to'         => 'btc',
    'fiat_equivalent'     => 'usd',   // optional
    'is_fixed_rate'       => false,   // optional
    'is_fee_paid_by_user' => false,  // optional
]);
echo $min['min_amount'] . ' ' . $min['fiat_equivalent'];
```

---

### 🧾 Invoices

#### `createInvoice($params)` — Create invoice URL → redirect customer

```php
$invoice = $np->createInvoice([
    'price_amount'       => 49.99,
    'price_currency'     => 'usd',
    'pay_currency'       => 'btc',  // optional
    'order_id'           => 'inv-001',
    'order_description'  => 'Premium',
    'success_url'        => 'https://yoursite.com/success',
    'cancel_url'         => 'https://yoursite.com/cancel',
    'partially_paid_url' => 'https://yoursite.com/partial',  // optional
    'is_fixed_rate'      => true,
    'is_fee_paid_by_user' => false,
]);
// Redirect customer to: $invoice['invoice_url']
```

#### `createInvoicePayment($params)` — Payment for existing invoice

```php
$payment = $np->createInvoicePayment([
    'iid'               => $invoiceId,
    'pay_currency'      => 'btc',
    'purchase_id'       => 'purchase-123',
    'order_description' => 'Item',
    'customer_email'    => 'user@example.com',
]);
```

---

### 💸 Payouts (JWT required)

```php
$auth = $np->getAuthToken($email, $password);
$token = $auth['token'];
```

#### `validatePayoutAddress($params)` — Validate address

```php
try {
    $np->validatePayoutAddress(['address' => '0x...', 'currency' => 'eth']);
    echo "Valid\n";
} catch (NowPayments\NowPaymentsError $e) {
    echo "Invalid\n";
}
```

#### `createPayout($params, $jwtToken)` — Mass payout

```php
$batch = $np->createPayout([
    'ipn_callback_url' => 'https://yoursite.com/payout-webhook',
    'withdrawals' => [
        ['address' => 'TEmGw...', 'currency' => 'trx', 'amount' => 200],
        ['address' => '0x1EB...', 'currency' => 'eth', 'amount' => 0.1],
        // Scheduled: ['address' => '...', 'currency' => 'trx', 'amount' => 100, 'execute_at' => '2024-12-31T10:00:00Z']
    ],
], $token);
// $batch['id'], $batch['withdrawals']
```

#### `verifyPayout($payoutId, $verificationCode, $jwtToken)` — 2FA verify

```php
$np->verifyPayout($batch['id'], '123456', $token);
```

#### `cancelPayout($payoutId, $jwtToken)` — Cancel scheduled payout

```php
// Use individual payout id, not batch id
$np->cancelPayout('5000000000', $token);
```

#### `getPayoutStatus($payoutId, $jwtToken?)` / `getPayouts($params?)`

```php
$status = $np->getPayoutStatus('5000000713', $token);

$payouts = $np->getPayouts([
    'batch_id'  => '5000000713',
    'status'    => 'finished',
    'limit'     => 10,
    'page'      => 0,
    'order_by'  => 'dateCreated',
    'order'     => 'desc',
]);
```

---

### 🏦 Fiat Payouts (JWT required)

#### `getFiatPayoutsCryptoCurrencies($params?, $jwtToken?)`

```php
$result = $np->getFiatPayoutsCryptoCurrencies(['provider' => 'transfi'], $token);
// $result['result']
```

#### `getFiatPayoutsPaymentMethods($params?, $jwtToken?)`

```php
$result = $np->getFiatPayoutsPaymentMethods(
    ['provider' => 'transfi', 'currency' => 'usd'],
    $token
);
```

#### `getFiatPayouts($params?, $jwtToken?)`

```php
$result = $np->getFiatPayouts([
    'status'   => 'FINISHED',
    'limit'    => 10,
    'dateFrom' => '2024-01-01',
], $token);
// $result['result']['rows']
```

---

### 💰 Balance

#### `getBalance($jwtToken?)`

```php
$balance = $np->getBalance($token);
// ['eth' => ['amount' => 0.5, 'pendingAmount' => 0], 'trx' => [...], ...]
```

---

### 🔄 Subscriptions (recurring)

#### `getSubscriptionPlans($params?)` / `getSubscriptionPlan($id)`

```php
$plans = $np->getSubscriptionPlans(['limit' => 10, 'offset' => 0]);
// $plans['result'], $plans['count']

$plan = $np->getSubscriptionPlan('76215585');
// $plan['result']
```

#### `updateSubscriptionPlan($id, $updates)`

```php
$np->updateSubscriptionPlan('76215585', [
    'amount'       => 9.99,
    'interval_day' => '30',
]);
```

#### `createSubscription($params, $jwtToken)` — Email or custody user

```php
// Email subscription:
$result = $np->createSubscription([
    'subscription_plan_id' => 76215585,
    'email' => 'customer@example.com',
], $token);

// Custody (sub-partner):
$result = $np->createSubscription([
    'subscription_plan_id' => 76215585,
    'sub_partner_id'      => '111394288',
], $token);
// $result['result']
```

#### `getSubscriptions($params?)` / `getSubscription($id)` / `deleteSubscription($id, $jwtToken?)`

```php
$list = $np->getSubscriptions([
    'status'               => 'PAID',
    'subscription_plan_id' => '111394288',
    'is_active'            => true,
    'limit'                => 10,
    'offset'               => 0,
]);

$sub = $np->getSubscription('1515573197');
$np->deleteSubscription('1515573197', $token);
```

---

### 👥 Custody / Sub-partners (JWT required)

#### `createSubPartner($name, $jwtToken)`

```php
$result = $np->createSubPartner('user-123', $token);
// $result['result']['id'], $result['result']['name']
```

#### `createSubPartnerPayment($params, $jwtToken)` — Top up sub-partner with crypto

```php
$result = $np->createSubPartnerPayment([
    'currency'       => 'trx',
    'amount'         => 50,
    'sub_partner_id' => '1631380403',
    'fixed_rate'     => false,
], $token);
// Show customer: Pay $result['result']['pay_amount'] TRX to $result['result']['pay_address']
```

#### `getSubPartners($params?, $jwtToken?)` / `getSubPartnerBalance($subPartnerId)`

```php
$users = $np->getSubPartners(['offset' => 0, 'limit' => 10, 'order' => 'DESC'], $token);

$balance = $np->getSubPartnerBalance('111394288');
// $balance['result']['balances']['usdtbsc']['amount']
```

#### `createTransfer($params, $jwtToken)` — Transfer between users

```php
$np->createTransfer([
    'currency' => 'trx',
    'amount'  => 0.3,
    'from_id' => 111394288,
    'to_id'   => 5209391548,
], $token);
```

#### `deposit($params, $jwtToken)` / `writeOff($params, $jwtToken)`

```php
$np->deposit([
    'currency'       => 'trx',
    'amount'         => 0.5,
    'sub_partner_id' => '111394288',
], $token);

$np->writeOff([
    'currency'       => 'trx',
    'amount'         => 0.3,
    'sub_partner_id' => '111394288',
], $token);
```

#### `getTransfers($params?, $jwtToken?)` / `getTransfer($id, $jwtToken?)`

```php
$transfers = $np->getTransfers(['status' => 'FINISHED', 'limit' => 10], $token);
$transfer = $np->getTransfer('327209161', $token);
```

---

### 🔀 Conversions (JWT required)

#### `createConversion($params, $jwtToken)`

```php
$np->createConversion([
    'amount'        => 50,
    'from_currency' => 'usdttrc20',
    'to_currency'   => 'usdterc20',
], $token);
```

#### `getConversionStatus($conversionId, $jwtToken)` / `getConversions($params?, $jwtToken?)`

```php
$status = $np->getConversionStatus('1327866232', $token);
$list = $np->getConversions(['status' => 'FINISHED', 'limit' => 10], $token);
```

---

### 🔔 IPN / Webhooks

```php
$np = new NowPayments([
    'apiKey'    => '...',
    'ipnSecret' => 'SECRET',
]);

// In your webhook handler (Laravel, Slim, plain PHP):
$payload   = file_get_contents('php://input');
$signature = $_SERVER['HTTP_X_NOWPAYMENTS_SIG'] ?? '';

if ($np->verifyIpn($payload, $signature)) {
    $data = json_decode($payload, true);
    // Process: $data['payment_id'], $data['payment_status'], etc.
}
```

**Standalone (without instance):**

```php
use NowPayments\Ipn;

$valid = Ipn::verifySignature($payload, $signature, 'SECRET');

// Create signature for testing:
$sig = Ipn::createSignature($payloadArray, 'SECRET');
```

---

### 🛠 Helper Functions

```php
use NowPayments\Helpers;
use NowPayments\Constants;

$payment = $np->getPaymentStatus($id);

// Human-readable status
Helpers::getStatusLabel($payment['payment_status']);
// "Awaiting payment" | "Completed" | "Failed" | ...

// Status checks
Helpers::isPaymentComplete($payment['payment_status']);  // finished, failed, refunded, expired
Helpers::isPaymentPending($payment['payment_status']);   // waiting, confirming, ...

// Summary string
Helpers::getPaymentSummary($payment);
// "Awaiting payment: 0.001234 BTC → bc1q..."

// Constants
Constants::PAYMENT_STATUS_LABELS;
Constants::PAYMENT_STATUSES;
Constants::PAYMENT_DONE_STATUSES;
Constants::PAYMENT_PENDING_STATUSES;
```

---

### ⚠️ Error Handling

```php
use NowPayments\NowPaymentsError;

try {
    $np->createPayment([...]);
} catch (NowPaymentsError $e) {
    echo $e->getMessage();      // "Invalid api key"
    echo $e->getStatusCode();   // 401
    echo $e->getErrorCode();    // API error code if any
    echo $e->getResponse();     // Raw response data
    echo (string) $e;          // "Invalid api key (status: 401)"
}
```

---

## 📁 Run Examples

Clone the repo and run from the project root:

| File | Description |
|------|-------------|
| `01-create-payment.php` | Create payment |
| `02-check-status.php` | Check payment status |
| `03-ipn-webhook.php` | IPN verification |

```bash
# Set your API key
export NOWPAYMENTS_API_KEY=your_key

# Run examples
php examples/01-create-payment.php
php examples/02-check-status.php 12345678
```

---

## 🔗 Links

| Link | URL |
|------|-----|
| **API Docs** | [Postman](https://documenter.getpostman.com/view/7907941/2s93JusNJt) |
| **Sandbox** | [Postman Sandbox](https://documenter.getpostman.com/view/7907941/T1LSCRHC) |
| **Help** | [nowpayments.io/help](https://nowpayments.io/help/payments/api) |
| **Dashboard** | [account.nowpayments.io](https://account.nowpayments.io) |
| **Node SDK** | [nowpayments-node](https://github.com/Foisalislambd/nowpayments-node) |

---

## 📄 License

MIT © [Foisalislambd](https://github.com/Foisalislambd)
