# nowpayments-go

<p align="center">
  <strong>Full-featured Go client for the NOWPayments cryptocurrency payment API</strong>
</p>

<p align="center">
  <a href="https://nowpayments.io">NOWPayments</a> •
  <a href="https://documenter.getpostman.com/view/7907941/2s93JusNJt">API Docs</a> •
  <a href="https://github.com/Foisalislambd/nowpayments-node">Node.js Port</a>
</p>

---

## Table of Contents

- [Installation](#installation)
- [Quick Start](#quick-start)
- [Configuration](#configuration)
- [API Examples](#api-examples)
  - [Status & Auth](#status--auth)
  - [Currencies](#currencies)
  - [Payments](#payments)
  - [Invoices](#invoices)
  - [Payouts](#payouts)
  - [Subscriptions](#subscriptions)
  - [Custody & Sub-Partners](#custody--sub-partners)
  - [Conversions](#conversions)
  - [Fiat Payouts](#fiat-payouts)
- [IPN Verification](#ipn-verification)
- [Payment Helpers](#payment-helpers)
- [Error Handling](#error-handling)
- [Types Reference](#types-reference)
- [License](#license)

---

## Installation

```bash
go get github.com/foisalislambd/nowpayments-go
```

**Requirements:** Go 1.21+

---

## Quick Start

```go
package main

import (
    "fmt"
    "log"

    "github.com/foisalislambd/nowpayments-go"
)

func main() {
    client, err := nowpayments.New(nowpayments.Config{
        APIKey:  "your-api-key",
        Sandbox: true,
    })
    if err != nil {
        log.Fatal(err)
    }

    // Check API status
    status, _ := client.GetStatus()
    fmt.Println("API:", status.Message)

    // Create payment
    payment, err := client.CreatePayment(nowpayments.CreatePaymentParams{
        PriceAmount:   29.99,
        PriceCurrency: "usd",
        PayCurrency:   "btc",
        OrderID:       "order-123",
    })
    if err != nil {
        log.Fatal(err)
    }
    fmt.Printf("Pay %g %s to %s\n", payment.PayAmount, payment.PayCurrency, payment.PayAddress)
}
```

---

## Configuration

| Field     | Type          | Required | Description                              |
| --------- | ------------- | -------- | ---------------------------------------- |
| `APIKey`  | `string`      | Yes      | Your NOWPayments API key                 |
| `Sandbox` | `bool`        | No       | Use sandbox API (default: `false`)       |
| `BaseURL` | `string`      | No       | Override base URL (overrides Sandbox)     |
| `Timeout` | `time.Duration` | No    | Request timeout (default: 30s)           |
| `IPNSecret` | `string`    | No       | For webhook signature verification       |

```go
client, err := nowpayments.New(nowpayments.Config{
    APIKey:    "your-api-key",
    Sandbox:   true,
    Timeout:   30 * time.Second,
    IPNSecret: "your-ipn-secret",
})
```

---

## API Examples

### Status & Auth

```go
// Check API availability
status, err := client.GetStatus()
// status: ApiStatusResponse{ Status, Message }

// Get JWT token (required for payouts, custody, conversions)
// Token expires in 5 minutes
auth, err := client.GetAuthToken("your@email.com", "password")
token := auth.Token
```

---

### Currencies

```go
// List available currencies
currencies, err := client.GetCurrencies(nil)
// currencies: map[string][]string{"currencies": ["btc","eth",...]}

// With fixed rate filter
fixedRate := true
currencies, err := client.GetCurrencies(&fixedRate)

// Full currency details (id, code, name, network, etc.)
fullCurrencies, err := client.GetFullCurrencies()
// fullCurrencies.Currencies: []FullCurrency

// Merchant checked currencies
merchantCoins, err := client.GetMerchantCoins(nil)

// Single currency details
currency, err := client.GetCurrency("btc")
```

---

### Payments

```go
// Get price estimate (fiat → crypto)
estimate, err := client.GetEstimatePrice(nowpayments.EstimateParams{
    Amount:       100,
    CurrencyFrom: "usd",
    CurrencyTo:   "btc",
})
// estimate: EstimatePriceResponse{ AmountFrom, CurrencyFrom, CurrencyTo, EstimatedAmount }

// Get minimum amount for pair
fixedRate := false
minAmount, err := client.GetMinAmount(nowpayments.MinAmountParams{
    CurrencyFrom: "usd",
    CurrencyTo:   "btc",
    IsFixedRate:  &fixedRate,
})
// minAmount: MinAmountResponse{ CurrencyFrom, CurrencyTo, MinAmount, FiatEquivalent }

// Create payment
fixedRate := true
payment, err := client.CreatePayment(nowpayments.CreatePaymentParams{
    PriceAmount:      29.99,
    PriceCurrency:    "usd",
    PayCurrency:     "btc",
    OrderID:         "order-123",
    OrderDescription: "Premium plan",
    IPNCallbackURL:  "https://yoursite.com/ipn",
    IsFixedRate:     &fixedRate,
})
// payment: Payment{ PaymentID, PaymentStatus, PayAddress, PayAmount, PayCurrency, ... }

// Get payment status
payment, err := client.GetPaymentStatus(12345678)

// List payments (paginated)
payments, err := client.GetPayments(&nowpayments.ListPaymentsParams{
    Limit:    10,
    Page:     1,
    SortBy:   "created_at",
    OrderBy:  "desc",
    DateFrom: "2024-01-01",
    DateTo:   "2024-12-31",
})
// payments: PaymentsListResponse{ Data, Limit, Page, PagesCount, Total }

// Update payment estimate (before expiration)
updated, err := client.UpdatePaymentEstimate(12345678)
// updated: UpdatePaymentEstimateResponse{ PayAmount, ExpirationEstimateDate, ID, TokenID }
```

---

### Invoices

```go
// Create invoice (redirect flow)
invoice, err := client.CreateInvoice(nowpayments.CreateInvoiceParams{
    PriceAmount:      99.99,
    PriceCurrency:    "usd",
    PayCurrency:     "btc",
    OrderID:         "inv-001",
    SuccessURL:      "https://yoursite.com/success",
    CancelURL:       "https://yoursite.com/cancel",
    IPNCallbackURL:  "https://yoursite.com/ipn",
})
// invoice: InvoiceResponse{ ID, InvoiceURL, PriceAmount, PriceCurrency, ... }

// Create payment for existing invoice
payment, err := client.CreateInvoicePayment(nowpayments.CreateInvoicePaymentParams{
    IID:          "invoice-id-or-number",
    PayCurrency:  "btc",
    CustomerEmail: "user@example.com",
})
```

---

### Payouts

```go
// Requires JWT - get via GetAuthToken first
auth, _ := client.GetAuthToken("email", "password")
token := auth.Token

// Create mass payout
payout, err := client.CreatePayout(nowpayments.CreatePayoutParams{
    Withdrawals: []nowpayments.PayoutWithdrawal{
        {
            Address:   "bc1q...",
            Currency:  "btc",
            Amount:    0.01,
            ExtraID:   "", // for XRP, XLM, etc.
        },
    },
    IPNCallbackURL: "https://yoursite.com/payout-ipn",
}, token)
// payout: CreatePayoutResponse{ ID, Withdrawals }

// Verify payout (2FA)
result, err := client.VerifyPayout("payout-id", "123456", token)

// Get payout status
status, err := client.GetPayoutStatus("payout-id", token)

// List payouts
payouts, err := client.GetPayouts(&nowpayments.GetPayoutsParams{
    Limit:    20,
    Page:     1,
    Status:   "FINISHED",
    DateFrom: "2024-01-01",
    DateTo:   "2024-12-31",
})

// Validate address before payout
valid, err := client.ValidatePayoutAddress(nowpayments.ValidateAddressParams{
    Address:  "bc1q...",
    Currency: "btc",
    ExtraID:  "",
})

// Cancel scheduled payout
err := client.CancelPayout("payout-id", token)
```

---

### Subscriptions

```go
// List subscription plans
plans, err := client.GetSubscriptionPlans(&nowpayments.GetSubscriptionPlansParams{
    Limit:  10,
    Offset: 0,
})
// plans: SubscriptionPlansResponse{ Count, Result []SubscriptionPlan }

// Get single plan
plan, err := client.GetSubscriptionPlan("plan-id")
// plan: SubscriptionPlanResponse{ Result SubscriptionPlan }

// Update plan
_, err := client.UpdateSubscriptionPlan("plan-id", map[string]interface{}{
    "title": "Updated Title",
    "amount": 19.99,
})

// List subscriptions (recurring payments)
isActive := true
subs, err := client.GetSubscriptions(&nowpayments.GetSubscriptionsParams{
    Status:             "active",
    SubscriptionPlanID: "plan-id",
    IsActive:           &isActive,
    Limit:              10,
    Offset:             0,
})
// subs: SubscriptionsResponse{ Count, Result []RecurringPayment }

// Get single subscription
sub, err := client.GetSubscription("sub-id")
// sub: SubscriptionResponse{ Result RecurringPayment }

// Create subscription (requires JWT)
sub, err := client.CreateSubscription(nowpayments.CreateSubscriptionParams{
    SubscriptionPlanID: "plan-id",
    Email:              "user@example.com",
    SubPartnerID:       "partner-id", // optional, for custody
}, token)
// sub: CreateSubscriptionResponse{ Result RecurringPayment }

// Cancel subscription
_, err := client.DeleteSubscription("sub-id", token)
```

---

### Custody & Sub-Partners

```go
// Get balance (JWT optional for some setups)
balance, err := client.GetBalance(token)

// List sub-partners
partners, err := client.GetSubPartners(&nowpayments.GetSubPartnersParams{
    Limit:  10,
    Offset: 0,
    Order:  "DESC",
}, token)

// Get sub-partner balance
balance, err := client.GetSubPartnerBalance("partner-id")
// balance: SubPartnerBalanceResponse{ Result SubPartnerBalance }

// Create sub-partner (requires JWT)
partner, err := client.CreateSubPartner("User Name", token)
// partner: CreateSubPartnerResponse{ Result{ ID, Name, CreatedAt, UpdatedAt } }

// Create sub-partner deposit payment (customer pays → funds go to partner)
fixedRate := true
payment, err := client.CreateSubPartnerPayment(nowpayments.CreateSubPartnerPaymentParams{
    Currency:     "btc",
    Amount:       0.001,
    SubPartnerID: "partner-id",
    FixedRate:    &fixedRate,
}, token)
// payment: SubPartnerPaymentResponse{ Result SubPartnerPaymentResult }

// Transfer between accounts
_, err := client.CreateTransfer(nowpayments.CreateTransferParams{
    Currency: "btc",
    Amount:   0.01,
    FromID:   "partner-1",
    ToID:     "partner-2",
}, token)

// Write off from user to master
_, err := client.WriteOff(nowpayments.WriteOffParams{
    Currency:     "btc",
    Amount:       0.001,
    SubPartnerID: "partner-id",
}, token)

// Deposit from master to user
_, err := client.Deposit(nowpayments.DepositParams{
    Currency:     "btc",
    Amount:       0.001,
    SubPartnerID: "partner-id",
}, token)

// List transfers
transfers, err := client.GetTransfers(&nowpayments.GetTransfersParams{
    Limit:  10,
    Offset: 0,
}, token)

// Get single transfer
transfer, err := client.GetTransfer("transfer-id", token)
```

---

### Conversions

```go
// Create conversion (requires JWT)
result, err := client.CreateConversion(nowpayments.CreateConversionParams{
    Amount:       100,
    FromCurrency: "btc",
    ToCurrency:   "usdt",
}, token)

// Get conversion status
status, err := client.GetConversionStatus("conversion-id", token)

// List conversions
conversions, err := client.GetConversions(&nowpayments.GetConversionsParams{
    ID:             []int{1, 2, 3},
    Status:         []string{"finished"},
    FromCurrency:   "btc",
    ToCurrency:     "usdt",
    CreatedAtFrom:  "2024-01-01",
    CreatedAtTo:    "2024-12-31",
    Limit:          10,
    Offset:        0,
    Order:         "DESC",
}, token)
```

---

### Fiat Payouts

```go
// Get crypto currencies for fiat cashout
cryptoCurrencies, err := client.GetFiatPayoutsCryptoCurrencies(
    &nowpayments.FiatPayoutsCryptoParams{
        Provider: "provider-name",
        Currency: "usd",
    },
    token,
)
// cryptoCurrencies: FiatPayoutsCryptoResponse{ Result []FiatPayoutCryptoCurrency }

// Get payment methods for fiat payout
methods, err := client.GetFiatPayoutsPaymentMethods(
    &nowpayments.FiatPayoutsCryptoParams{Provider: "x", Currency: "usd"},
    token,
)
// methods: FiatPayoutsPaymentMethodsResponse{ Result []FiatPayoutPaymentMethod }

// List fiat payouts
payouts, err := client.GetFiatPayouts(&nowpayments.GetFiatPayoutsParams{
    Provider:         "x",
    Status:           "FINISHED",
    FiatCurrency:     "usd",
    CryptoCurrency:   "btc",
    Limit:            10,
    Page:             1,
    DateFrom:         "2024-01-01",
    DateTo:           "2024-12-31",
}, token)
// payouts: FiatPayoutsResponse{ Result{ Rows []FiatPayoutRecord } }
```

---

## IPN Verification

Verify webhook signatures from NOWPayments (HMAC-SHA512):

```go
// Using client's configured IPNSecret
valid, err := client.VerifyIPN(reqBody, req.Header.Get("x-nowpayments-sig"))
if valid {
    // Process payment notification
}

// Standalone with explicit secret
valid := nowpayments.VerifyIPNSignature(payload, signature, ipnSecret)

// Create signature for testing (e.g., mocking)
sig := nowpayments.CreateIPNSignature(
    map[string]interface{}{"payment_id": 123, "payment_status": "finished"},
    ipnSecret,
)
```

---

## Payment Helpers

```go
// Check if payment is complete (finished, failed, refunded, expired)
if nowpayments.IsPaymentComplete(payment.PaymentStatus) {
    // Terminal state
}

// Check if still pending
if nowpayments.IsPaymentPending(payment.PaymentStatus) {
    // Customer should pay
}

// Human-readable status label
label := nowpayments.GetStatusLabel(payment.PaymentStatus)
// "Awaiting payment", "Completed", "Failed", etc.

// Display summary
summary := nowpayments.GetPaymentSummary(payment)
// "Awaiting payment: 0.001234 BTC → bc1q..."

// Constants
nowpayments.PaymentStatuses       // All statuses
nowpayments.PaymentDoneStatuses   // Terminal statuses
nowpayments.PaymentPendingStatuses
nowpayments.PaymentStatusLabels   // map[PaymentStatus]string
```

---

## Error Handling

```go
payment, err := client.CreatePayment(params)
if err != nil {
    if npErr, ok := err.(*nowpayments.NowPaymentsError); ok {
        fmt.Println(npErr.Message)      // "Invalid API key"
        fmt.Println(npErr.StatusCode)   // 401
        fmt.Println(npErr.Code)         // "UNAUTHORIZED"
        fmt.Println(npErr.Response)     // Raw response body
    }
    return err
}
```

---

## Types Reference

### Core Types

| Type | Description |
|------|-------------|
| `Config` | Client configuration |
| `Client` | API client |
| `NowPaymentsError` | API/network error |
| `PaymentStatus` | `waiting`, `confirming`, `confirmed`, `spending`, `sending`, `partially_paid`, `finished`, `failed`, `refunded`, `expired` |
| `Payment` | Payment object from API |
| `ApiStatusResponse` | Status check response |
| `AuthResponse` | JWT token response |

### Request Params

| Type | Use |
|------|-----|
| `CreatePaymentParams` | Create payment |
| `CreateInvoiceParams` | Create invoice |
| `CreateInvoicePaymentParams` | Create invoice payment |
| `CreatePayoutParams` | Mass payout |
| `PayoutWithdrawal` | Single withdrawal in payout |
| `ValidateAddressParams` | Validate address |
| `CreateSubPartnerPaymentParams` | Sub-partner deposit |
| `CreateTransferParams` | Transfer between accounts |
| `WriteOffParams` | Write off from user |
| `DepositParams` | Deposit to user |
| `CreateConversionParams` | Create conversion |
| `CreateSubscriptionParams` | Create subscription |
| `EstimateParams` | Price estimate |
| `MinAmountParams` | Min amount |
| `ListPaymentsParams` | List payments |
| `GetPayoutsParams` | List payouts |
| `GetSubscriptionsParams` | List subscriptions |
| `GetSubscriptionPlansParams` | List plans |
| `GetSubPartnersParams` | List sub-partners |
| `GetTransfersParams` | List transfers |
| `GetConversionsParams` | List conversions |
| `GetFiatPayoutsParams` | List fiat payouts |
| `GetFiatPayoutsCryptoParams` | Fiat crypto currencies params |

### Response Types

| Type | Use |
|------|-----|
| `PaymentsListResponse` | Paginated payments |
| `EstimatePriceResponse` | Price estimate |
| `MinAmountResponse` | Min amount |
| `InvoiceResponse` | Create invoice |
| `CreatePayoutResponse` | Create payout |
| `PayoutWithdrawalItem` | Single withdrawal in response |
| `SubPartnerBalance` | Sub-partner balance |
| `SubPartnerBalanceResponse` | Get balance |
| `SubPartnerPaymentResponse` | Sub-partner payment |
| `CreateSubPartnerResponse` | Create sub-partner |
| `FullCurrency` | Currency details |
| `SubscriptionPlan` | Subscription plan |
| `RecurringPayment` | Recurring payment |
| `SubscriptionPlanResponse` | Get single plan |
| `SubscriptionPlansResponse` | List plans |
| `SubscriptionResponse` | Get single subscription |
| `SubscriptionsResponse` | List subscriptions |
| `CreateSubscriptionResponse` | Create subscription |
| `DeleteSubscriptionResponse` | Delete subscription |
| `UpdatePaymentEstimateResponse` | Update estimate |
| `FiatPayoutCryptoCurrency` | Fiat crypto option |
| `FiatPayoutPaymentMethod` | Fiat payment method |
| `FiatPayoutRecord` | Fiat payout record |
| `FiatPayoutsCryptoResponse` | Fiat crypto currencies |
| `FiatPayoutsPaymentMethodsResponse` | Fiat payment methods |
| `FiatPayoutsResponse` | List fiat payouts |

---

## License

MIT © [Foisalislambd](https://github.com/Foisalislambd)
