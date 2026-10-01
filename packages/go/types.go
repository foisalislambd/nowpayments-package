// Package nowpayments provides a full-featured Go client for the NOWPayments cryptocurrency payment API.
package nowpayments

// PaymentStatus represents the status of a payment.
type PaymentStatus string

const (
	StatusWaiting       PaymentStatus = "waiting"
	StatusConfirming    PaymentStatus = "confirming"
	StatusConfirmed     PaymentStatus = "confirmed"
	StatusSpending      PaymentStatus = "spending"
	StatusSending       PaymentStatus = "sending"
	StatusPartiallyPaid PaymentStatus = "partially_paid"
	StatusFinished      PaymentStatus = "finished"
	StatusFailed        PaymentStatus = "failed"
	StatusRefunded      PaymentStatus = "refunded"
	StatusExpired       PaymentStatus = "expired"
)

// PaymentStatuses lists all possible payment statuses.
var PaymentStatuses = []PaymentStatus{
	StatusWaiting, StatusConfirming, StatusConfirmed, StatusSpending,
	StatusSending, StatusPartiallyPaid, StatusFinished, StatusFailed,
	StatusRefunded, StatusExpired,
}

// PaymentDoneStatuses are terminal statuses (success or failure).
var PaymentDoneStatuses = []PaymentStatus{
	StatusFinished, StatusFailed, StatusRefunded, StatusExpired,
}

// PaymentPendingStatuses mean the customer should still pay.
var PaymentPendingStatuses = []PaymentStatus{
	StatusWaiting, StatusConfirming, StatusConfirmed, StatusSpending,
	StatusSending, StatusPartiallyPaid,
}

// CreatePaymentParams for creating a payment.
type CreatePaymentParams struct {
	PriceAmount       float64 `json:"price_amount"`
	PriceCurrency     string  `json:"price_currency"`
	PayCurrency       string  `json:"pay_currency"`
	PayAmount         float64 `json:"pay_amount,omitempty"`
	IPNCallbackURL    string  `json:"ipn_callback_url,omitempty"`
	OrderID           string  `json:"order_id,omitempty"`
	OrderDescription  string  `json:"order_description,omitempty"`
	PurchaseID        string  `json:"purchase_id,omitempty"`
	PayoutAddress     string  `json:"payout_address,omitempty"`
	PayoutCurrency    string  `json:"payout_currency,omitempty"`
	PayoutExtraID     string  `json:"payout_extra_id,omitempty"`
	FixedRate         *bool   `json:"fixed_rate,omitempty"` // Deprecated, use IsFixedRate
	IsFixedRate       *bool   `json:"is_fixed_rate,omitempty"`
	IsFeePaidByUser   *bool   `json:"is_fee_paid_by_user,omitempty"`
}

// CreateInvoiceParams for creating an invoice.
type CreateInvoiceParams struct {
	PriceAmount       float64 `json:"price_amount"`
	PriceCurrency     string  `json:"price_currency"`
	PayCurrency       string  `json:"pay_currency,omitempty"`
	IPNCallbackURL    string  `json:"ipn_callback_url,omitempty"`
	OrderID           string  `json:"order_id,omitempty"`
	OrderDescription  string  `json:"order_description,omitempty"`
	SuccessURL        string  `json:"success_url,omitempty"`
	CancelURL         string  `json:"cancel_url,omitempty"`
	PartiallyPaidURL  string  `json:"partially_paid_url,omitempty"`
	IsFixedRate       *bool   `json:"is_fixed_rate,omitempty"`
	IsFeePaidByUser   *bool   `json:"is_fee_paid_by_user,omitempty"`
}

// CreateInvoicePaymentParams for creating payment for existing invoice.
type CreateInvoicePaymentParams struct {
	IID               interface{} `json:"iid"` // int or string
	PayCurrency       string      `json:"pay_currency,omitempty"`
	PurchaseID        string      `json:"purchase_id,omitempty"`
	OrderDescription  string      `json:"order_description,omitempty"`
	CustomerEmail     string      `json:"customer_email,omitempty"`
	PayoutAddress     string      `json:"payout_address,omitempty"`
	PayoutExtraID     string      `json:"payout_extra_id,omitempty"`
	PayoutCurrency    string      `json:"payout_currency,omitempty"`
}

// FullCurrency from GET /v1/full-currencies.
type FullCurrency struct {
	ID               int     `json:"id"`
	Code             string  `json:"code"`
	Name             string  `json:"name"`
	Enable           bool    `json:"enable"`
	WalletRegex      string  `json:"wallet_regex,omitempty"`
	Priority         int     `json:"priority,omitempty"`
	ExtraIDExists    bool    `json:"extra_id_exists,omitempty"`
	ExtraIDRegex     *string `json:"extra_id_regex,omitempty"`
	LogoURL          string  `json:"logo_url,omitempty"`
	Track            bool    `json:"track,omitempty"`
	CgID             string  `json:"cg_id,omitempty"`
	IsMaxlimit       bool    `json:"is_maxlimit,omitempty"`
	Network          string  `json:"network,omitempty"`
	SmartContract    *string `json:"smart_contract,omitempty"`
	NetworkPrecision *int    `json:"network_precision,omitempty"`
}

// FiatPayoutCryptoCurrency for fiat payout options.
type FiatPayoutCryptoCurrency struct {
	Provider         string `json:"provider"`
	CurrencyCode     string `json:"currencyCode"`
	CurrencyNetwork  string `json:"currencyNetwork"`
	Enabled          bool   `json:"enabled"`
}

// FiatPayoutField for payment method fields.
type FiatPayoutField struct {
	Name        string `json:"name"`
	Type        string `json:"type"`
	Mandatory   bool   `json:"mandatory"`
	Description string `json:"description,omitempty"`
}

// FiatPayoutPaymentMethod for fiat payout.
type FiatPayoutPaymentMethod struct {
	Name         string              `json:"name"`
	PaymentCode  string              `json:"paymentCode"`
	Fields       []FiatPayoutField    `json:"fields"`
	Provider     string              `json:"provider"`
}

// FiatPayoutRecord for fiat payout list.
type FiatPayoutRecord struct {
	ID                   string  `json:"id"`
	Provider             string  `json:"provider"`
	RequestID            string  `json:"requestId"`
	Status               string  `json:"status"`
	FiatCurrencyCode     string  `json:"fiatCurrencyCode,omitempty"`
	FiatAmount           string  `json:"fiatAmount,omitempty"`
	CryptoCurrencyCode   string  `json:"cryptoCurrencyCode,omitempty"`
	CryptoCurrencyAmount string  `json:"cryptoCurrencyAmount,omitempty"`
	FiatAccountCode      string  `json:"fiatAccountCode,omitempty"`
	FiatAccountNumber    string  `json:"fiatAccountNumber,omitempty"`
	PayoutDescription   *string `json:"payoutDescription,omitempty"`
	Error                *string `json:"error,omitempty"`
	CreatedAt            string  `json:"createdAt,omitempty"`
	UpdatedAt            string  `json:"updatedAt,omitempty"`
}

// GetSubscriptionPlansParams for listing subscription plans.
type GetSubscriptionPlansParams struct {
	Limit  int `json:"limit,omitempty"`
	Offset int `json:"offset,omitempty"`
}

// GetSubscriptionsParams for listing subscriptions.
type GetSubscriptionsParams struct {
	Status             string      `json:"status,omitempty"`
	SubscriptionPlanID interface{} `json:"subscription_plan_id,omitempty"`
	IsActive           *bool       `json:"is_active,omitempty"`
	Limit              int         `json:"limit,omitempty"`
	Offset             int         `json:"offset,omitempty"`
}

// GetFiatPayoutsParams for listing fiat payouts.
type GetFiatPayoutsParams struct {
	ID               string `json:"id,omitempty"`
	Provider         string `json:"provider,omitempty"`
	RequestID        string `json:"requestId,omitempty"`
	FiatCurrency     string `json:"fiatCurrency,omitempty"`
	CryptoCurrency   string `json:"cryptoCurrency,omitempty"`
	Status           string `json:"status,omitempty"`
	Filter           string `json:"filter,omitempty"`
	ProviderPayoutID string `json:"provider_payout_id,omitempty"`
	Limit            int    `json:"limit,omitempty"`
	Page             int    `json:"page,omitempty"`
	OrderBy          string `json:"orderBy,omitempty"`
	SortBy           string `json:"sortBy,omitempty"`
	DateFrom         string `json:"dateFrom,omitempty"`
	DateTo           string `json:"dateTo,omitempty"`
}

// Payment from API.
type Payment struct {
	PaymentID      interface{}   `json:"payment_id"` // int or string
	PaymentStatus  PaymentStatus `json:"payment_status"`
	PayAddress     string        `json:"pay_address"`
	PayAmount      float64       `json:"pay_amount"`
	PayCurrency    string        `json:"pay_currency"`
	PriceAmount    float64       `json:"price_amount"`
	PriceCurrency  string        `json:"price_currency"`
	ActuallyPaid   float64       `json:"actually_paid,omitempty"`
	OutcomeAmount  float64       `json:"outcome_amount,omitempty"`
	OutcomeCurrency string       `json:"outcome_currency,omitempty"`
	OrderID        string        `json:"order_id,omitempty"`
	OrderDescription string      `json:"order_description,omitempty"`
	PurchaseID     string        `json:"purchase_id,omitempty"`
	CreatedAt      string        `json:"created_at,omitempty"`
	UpdatedAt      string        `json:"updated_at,omitempty"`
}

// PaymentsListResponse paginated payments.
type PaymentsListResponse struct {
	Data       []Payment `json:"data"`
	Limit      int       `json:"limit"`
	Page       int       `json:"page"`
	PagesCount int       `json:"pagesCount"`
	Total      int       `json:"total"`
}

// EstimatePriceResponse from estimate endpoint.
type EstimatePriceResponse struct {
	AmountFrom     float64 `json:"amount_from"`
	CurrencyFrom   string  `json:"currency_from"`
	CurrencyTo     string  `json:"currency_to"`
	EstimatedAmount float64 `json:"estimated_amount"`
}

// MinAmountResponse from min-amount endpoint.
type MinAmountResponse struct {
	CurrencyFrom   string   `json:"currency_from"`
	CurrencyTo     string   `json:"currency_to"`
	MinAmount      float64  `json:"min_amount"`
	FiatEquivalent *float64 `json:"fiat_equivalent,omitempty"`
}

// ListPaymentsParams for listing payments.
type ListPaymentsParams struct {
	Limit    int    `json:"limit,omitempty"`
	Page     int    `json:"page,omitempty"`
	SortBy   string `json:"sortBy,omitempty"`
	OrderBy  string `json:"orderBy,omitempty"`
	DateFrom string `json:"dateFrom,omitempty"`
	DateTo   string `json:"dateTo,omitempty"`
}

// EstimateParams for price estimate.
type EstimateParams struct {
	Amount       float64 `json:"amount"`
	CurrencyFrom string  `json:"currency_from"`
	CurrencyTo   string  `json:"currency_to"`
}

// MinAmountParams for min amount.
type MinAmountParams struct {
	CurrencyFrom     string  `json:"currency_from"`
	CurrencyTo       string  `json:"currency_to"`
	FiatEquivalent   interface{} `json:"fiat_equivalent,omitempty"` // string or bool
	IsFixedRate      *bool   `json:"is_fixed_rate,omitempty"`
	IsFeePaidByUser  *bool   `json:"is_fee_paid_by_user,omitempty"`
}

// InvoiceResponse from create invoice.
type InvoiceResponse struct {
	ID               string  `json:"id"`
	InvoiceID        string  `json:"invoice_id,omitempty"`
	InvoiceURL       string  `json:"invoice_url"`
	PriceAmount      float64 `json:"price_amount"`
	PriceCurrency    string  `json:"price_currency"`
	PayCurrency      string  `json:"pay_currency,omitempty"`
	OrderID          string  `json:"order_id,omitempty"`
	OrderDescription string  `json:"order_description,omitempty"`
}

// SubscriptionPlan for recurring payments.
type SubscriptionPlan struct {
	ID          string  `json:"id"`
	Amount      float64 `json:"amount"`
	Currency    string  `json:"currency"`
	IntervalDay string  `json:"interval_day"`
	Title       string  `json:"title"`
	CreatedAt   string  `json:"created_at,omitempty"`
	UpdatedAt   string  `json:"updated_at,omitempty"`
}

// RecurringPayment subscription.
type RecurringPayment struct {
	ID                 string   `json:"id"`
	SubscriptionPlanID string   `json:"subscription_plan_id"`
	Status             string   `json:"status"`
	IsActive           bool     `json:"is_active"`
	CreatedAt          string   `json:"created_at,omitempty"`
	UpdatedAt          string   `json:"updated_at,omitempty"`
	ExpireDate         string   `json:"expire_date,omitempty"`
	Subscriber         *struct {
		Email        string `json:"email,omitempty"`
		SubPartnerID string `json:"sub_partner_id,omitempty"`
	} `json:"subscriber,omitempty"`
}

// SubPartnerBalanceItem for balance per currency.
type SubPartnerBalanceItem struct {
	Amount        float64 `json:"amount"`
	PendingAmount float64 `json:"pendingAmount"`
}

// SubPartnerBalance response.
type SubPartnerBalance struct {
	SubPartnerID string                          `json:"subPartnerId"`
	Balances     map[string]SubPartnerBalanceItem `json:"balances"`
}

// PayoutWithdrawal for create payout.
type PayoutWithdrawal struct {
	Address           string   `json:"address"`
	Currency          string   `json:"currency"`
	Amount            float64  `json:"amount"`
	ExtraID           string   `json:"extra_id,omitempty"`
	IPNCallbackURL     string   `json:"ipn_callback_url,omitempty"`
	PayoutDescription string   `json:"payout_description,omitempty"`
	UniqueExternalID  string   `json:"unique_external_id,omitempty"`
	FiatAmount        float64  `json:"fiat_amount,omitempty"`
	FiatCurrency      string   `json:"fiat_currency,omitempty"`
	ExecuteAt         string   `json:"execute_at,omitempty"` // ISO date for scheduled payout
}

// CreatePayoutParams for mass payout.
type CreatePayoutParams struct {
	IPNCallbackURL     string             `json:"ipn_callback_url,omitempty"`
	PayoutDescription  string             `json:"payout_description,omitempty"`
	Withdrawals        []PayoutWithdrawal `json:"withdrawals"`
}

// PayoutWithdrawalItem in create payout response.
type PayoutWithdrawalItem struct {
	ID                string `json:"id"`
	Address           string `json:"address"`
	Currency          string `json:"currency"`
	Amount            string `json:"amount"`
	Status            string `json:"status"`
	BatchWithdrawalID string `json:"batch_withdrawal_id"`
}

// CreatePayoutResponse from create payout.
type CreatePayoutResponse struct {
	ID          string                `json:"id"`
	Withdrawals []PayoutWithdrawalItem `json:"withdrawals"`
}

// AuthResponse JWT token.
type AuthResponse struct {
	Token string `json:"token"`
}

// ValidateAddressParams for address validation.
type ValidateAddressParams struct {
	Address  string `json:"address"`
	Currency string `json:"currency"`
	ExtraID  string `json:"extra_id,omitempty"`
}

// CreateSubPartnerPaymentParams for sub-partner deposit.
type CreateSubPartnerPaymentParams struct {
	Currency      string      `json:"currency"`
	Amount        float64     `json:"amount"`
	SubPartnerID  interface{} `json:"sub_partner_id"` // string or int
	FixedRate     *bool       `json:"fixed_rate,omitempty"`
}

// SubPartnerPaymentResult extended payment for sub-partner.
type SubPartnerPaymentResult struct {
	Payment
	AmountReceived         float64  `json:"amount_received,omitempty"`
	IPNCallbackURL         *string  `json:"ipn_callback_url,omitempty"`
	SmartContract         *string  `json:"smart_contract,omitempty"`
	Network               string   `json:"network,omitempty"`
	NetworkPrecision      *int     `json:"network_precision,omitempty"`
	TimeLimit             *int     `json:"time_limit,omitempty"`
	BurningPercent        *float64 `json:"burning_percent,omitempty"`
	ExpirationEstimateDate string  `json:"expiration_estimate_date,omitempty"`
}

// SubPartnerPaymentResponse from create sub-partner payment.
type SubPartnerPaymentResponse struct {
	Result SubPartnerPaymentResult `json:"result"`
}

// ApiStatusResponse from GET /v1/status.
type ApiStatusResponse struct {
	Status  string `json:"status,omitempty"`
	Message string `json:"message,omitempty"`
}

// UpdatePaymentEstimateResponse from update-merchant-estimate.
type UpdatePaymentEstimateResponse struct {
	PayAmount              float64 `json:"pay_amount"`
	ExpirationEstimateDate string  `json:"expiration_estimate_date"`
	ID                     string  `json:"id"`
	TokenID                string  `json:"token_id"`
}

// CreateConversionParams for creating a conversion.
type CreateConversionParams struct {
	Amount       float64 `json:"amount"`
	FromCurrency string  `json:"from_currency"`
	ToCurrency   string  `json:"to_currency"`
}

// CreateSubPartnerResponse from create sub-partner.
type CreateSubPartnerResponse struct {
	Result struct {
		ID        string `json:"id"`
		Name      string `json:"name"`
		CreatedAt string `json:"created_at"`
		UpdatedAt string `json:"updated_at"`
	} `json:"result"`
}
