package nowpayments

import (
	"fmt"
	"net/url"
	"strings"
)

// GetSubPartners returns list of sub-partners. JWT optional.
func (c *Client) GetSubPartners(params *GetSubPartnersParams, jwtToken string) (interface{}, error) {
	q := url.Values{}
	if params != nil {
		if params.ID != nil {
			appendQueryParam(q, "id", params.ID)
		}
		if params.Offset > 0 {
			q.Set("offset", fmt.Sprintf("%d", params.Offset))
		}
		if params.Limit > 0 {
			q.Set("limit", fmt.Sprintf("%d", params.Limit))
		}
		if params.Order != "" {
			q.Set("order", params.Order)
		}
	}
	path := "/v1/sub-partner"
	if len(q) > 0 {
		path += "?" + q.Encode()
	}
	var result interface{}
	if err := c.get(path, nil, &result, jwtToken); err != nil {
		return nil, err
	}
	return result, nil
}

// GetSubPartnersParams for listing sub-partners.
type GetSubPartnersParams struct {
	ID     interface{} `json:"id,omitempty"` // number or []number
	Offset int         `json:"offset,omitempty"`
	Limit  int         `json:"limit,omitempty"`
	Order  string      `json:"order,omitempty"` // ASC or DESC
}

// GetSubPartnerBalance returns sub-partner balance.
func (c *Client) GetSubPartnerBalance(subPartnerID string) (*SubPartnerBalanceResponse, error) {
	var result SubPartnerBalanceResponse
	if err := c.get("/v1/sub-partner/balance/"+url.PathEscape(subPartnerID), nil, &result, ""); err != nil {
		return nil, err
	}
	return &result, nil
}

// SubPartnerBalanceResponse from get sub-partner balance.
type SubPartnerBalanceResponse struct {
	Result SubPartnerBalance `json:"result"`
}

// GetTransfersParams for listing transfers.
type GetTransfersParams struct {
	ID     interface{} `json:"id,omitempty"`
	Status interface{} `json:"status,omitempty"`
	Limit  int         `json:"limit,omitempty"`
	Offset int         `json:"offset,omitempty"`
	Order  string      `json:"order,omitempty"`
}

// GetTransfers returns list of transfers. JWT optional.
func (c *Client) GetTransfers(params *GetTransfersParams, jwtToken string) (interface{}, error) {
	q := url.Values{}
	if params != nil {
		if params.ID != nil {
			appendQueryParam(q, "id", params.ID)
		}
		if params.Status != nil {
			appendQueryParam(q, "status", params.Status)
		}
		if params.Limit > 0 {
			q.Set("limit", fmt.Sprintf("%d", params.Limit))
		}
		if params.Offset > 0 {
			q.Set("offset", fmt.Sprintf("%d", params.Offset))
		}
		if params.Order != "" {
			q.Set("order", params.Order)
		}
	}
	path := "/v1/sub-partner/transfers"
	if len(q) > 0 {
		path += "?" + q.Encode()
	}
	var result interface{}
	if err := c.get(path, nil, &result, jwtToken); err != nil {
		return nil, err
	}
	return result, nil
}

// GetTransfer returns single transfer. JWT optional.
func (c *Client) GetTransfer(id, jwtToken string) (interface{}, error) {
	var result interface{}
	if err := c.get("/v1/sub-partner/transfer/"+url.PathEscape(id), nil, &result, jwtToken); err != nil {
		return nil, err
	}
	return result, nil
}

// GetBalance returns custody balance. JWT optional.
func (c *Client) GetBalance(jwtToken string) (interface{}, error) {
	var result interface{}
	if err := c.get("/v1/balance", nil, &result, jwtToken); err != nil {
		return nil, err
	}
	return result, nil
}

// CreateSubPartner creates new sub-partner. Requires JWT.
func (c *Client) CreateSubPartner(name, jwtToken string) (*CreateSubPartnerResponse, error) {
	if strings.TrimSpace(jwtToken) == "" {
		return nil, &NowPaymentsError{Message: "JWT token is required for CreateSubPartner. Call GetAuthToken first."}
	}
	body := map[string]string{"name": name}
	var result CreateSubPartnerResponse
	if err := c.post("/v1/sub-partner/balance", body, &result, jwtToken); err != nil {
		return nil, err
	}
	return &result, nil
}

// CreateSubPartnerPayment creates deposit payment for sub-partner. Requires JWT.
func (c *Client) CreateSubPartnerPayment(params CreateSubPartnerPaymentParams, jwtToken string) (*SubPartnerPaymentResponse, error) {
	if strings.TrimSpace(jwtToken) == "" {
		return nil, &NowPaymentsError{Message: "JWT token is required for CreateSubPartnerPayment. Call GetAuthToken first."}
	}
	var result SubPartnerPaymentResponse
	if err := c.post("/v1/sub-partner/payment", params, &result, jwtToken); err != nil {
		return nil, err
	}
	return &result, nil
}

// CreateTransferParams for transfer between accounts.
type CreateTransferParams struct {
	Currency string      `json:"currency"`
	Amount   float64     `json:"amount"`
	FromID   interface{} `json:"from_id"`
	ToID     interface{} `json:"to_id"`
}

// CreateTransfer transfers between user accounts. Requires JWT.
func (c *Client) CreateTransfer(params CreateTransferParams, jwtToken string) (interface{}, error) {
	if strings.TrimSpace(jwtToken) == "" {
		return nil, &NowPaymentsError{Message: "JWT token is required for CreateTransfer. Call GetAuthToken first."}
	}
	var result interface{}
	if err := c.post("/v1/sub-partner/transfer", params, &result, jwtToken); err != nil {
		return nil, err
	}
	return result, nil
}

// WriteOffParams for write off from user to master.
type WriteOffParams struct {
	Currency      string      `json:"currency"`
	Amount        float64     `json:"amount"`
	SubPartnerID  interface{} `json:"sub_partner_id"`
}

// WriteOff writes off from user to master account. Requires JWT.
func (c *Client) WriteOff(params WriteOffParams, jwtToken string) (interface{}, error) {
	if strings.TrimSpace(jwtToken) == "" {
		return nil, &NowPaymentsError{Message: "JWT token is required for WriteOff. Call GetAuthToken first."}
	}
	var result interface{}
	if err := c.post("/v1/sub-partner/write-off", params, &result, jwtToken); err != nil {
		return nil, err
	}
	return result, nil
}

// DepositParams for deposit from master to user.
type DepositParams struct {
	Currency     string      `json:"currency"`
	Amount       float64     `json:"amount"`
	SubPartnerID interface{} `json:"sub_partner_id"`
}

// Deposit deposits from master to user account. Requires JWT.
func (c *Client) Deposit(params DepositParams, jwtToken string) (interface{}, error) {
	if strings.TrimSpace(jwtToken) == "" {
		return nil, &NowPaymentsError{Message: "JWT token is required for Deposit. Call GetAuthToken first."}
	}
	var result interface{}
	if err := c.post("/v1/sub-partner/deposit", params, &result, jwtToken); err != nil {
		return nil, err
	}
	return result, nil
}

// GetFiatPayoutsCryptoCurrencies returns crypto currencies for fiat cashout. JWT optional.
func (c *Client) GetFiatPayoutsCryptoCurrencies(params *FiatPayoutsCryptoParams, jwtToken string) (*FiatPayoutsCryptoResponse, error) {
	q := url.Values{}
	if params != nil {
		if params.Provider != "" {
			q.Set("provider", params.Provider)
		}
		if params.Currency != "" {
			q.Set("currency", params.Currency)
		}
	}
	path := "/v1/fiat-payouts/crypto-currencies"
	if len(q) > 0 {
		path += "?" + q.Encode()
	}
	var result FiatPayoutsCryptoResponse
	if err := c.get(path, nil, &result, jwtToken); err != nil {
		return nil, err
	}
	return &result, nil
}

// FiatPayoutsCryptoParams for fiat payouts crypto.
type FiatPayoutsCryptoParams struct {
	Provider string `json:"provider,omitempty"`
	Currency string `json:"currency,omitempty"`
}

// FiatPayoutsCryptoResponse from get fiat payouts crypto currencies.
type FiatPayoutsCryptoResponse struct {
	Result []FiatPayoutCryptoCurrency `json:"result"`
}

// GetFiatPayoutsPaymentMethods returns payment methods for fiat payout. JWT optional.
func (c *Client) GetFiatPayoutsPaymentMethods(params *FiatPayoutsCryptoParams, jwtToken string) (*FiatPayoutsPaymentMethodsResponse, error) {
	q := url.Values{}
	if params != nil {
		if params.Provider != "" {
			q.Set("provider", params.Provider)
		}
		if params.Currency != "" {
			q.Set("currency", params.Currency)
		}
	}
	path := "/v1/fiat-payouts/payment-methods"
	if len(q) > 0 {
		path += "?" + q.Encode()
	}
	var result FiatPayoutsPaymentMethodsResponse
	if err := c.get(path, nil, &result, jwtToken); err != nil {
		return nil, err
	}
	return &result, nil
}

// FiatPayoutsPaymentMethodsResponse from get fiat payouts payment methods.
type FiatPayoutsPaymentMethodsResponse struct {
	Result []FiatPayoutPaymentMethod `json:"result"`
}

// GetFiatPayouts returns list of fiat payouts. JWT optional.
func (c *Client) GetFiatPayouts(params *GetFiatPayoutsParams, jwtToken string) (*FiatPayoutsResponse, error) {
	q := url.Values{}
	if params != nil {
		if params.ID != "" {
			q.Set("id", params.ID)
		}
		if params.Provider != "" {
			q.Set("provider", params.Provider)
		}
		if params.RequestID != "" {
			q.Set("requestId", params.RequestID)
		}
		if params.FiatCurrency != "" {
			q.Set("fiatCurrency", params.FiatCurrency)
		}
		if params.CryptoCurrency != "" {
			q.Set("cryptoCurrency", params.CryptoCurrency)
		}
		if params.Status != "" {
			q.Set("status", params.Status)
		}
		if params.Filter != "" {
			q.Set("filter", params.Filter)
		}
		if params.ProviderPayoutID != "" {
			q.Set("provider_payout_id", params.ProviderPayoutID)
		}
		if params.Limit > 0 {
			q.Set("limit", fmt.Sprintf("%d", params.Limit))
		}
		if params.Page > 0 {
			q.Set("page", fmt.Sprintf("%d", params.Page))
		}
		if params.OrderBy != "" {
			q.Set("orderBy", params.OrderBy)
		}
		if params.SortBy != "" {
			q.Set("sortBy", params.SortBy)
		}
		if params.DateFrom != "" {
			q.Set("dateFrom", params.DateFrom)
		}
		if params.DateTo != "" {
			q.Set("dateTo", params.DateTo)
		}
	}
	path := "/v1/fiat-payouts"
	if len(q) > 0 {
		path += "?" + q.Encode()
	}
	var result FiatPayoutsResponse
	if err := c.get(path, nil, &result, jwtToken); err != nil {
		return nil, err
	}
	return &result, nil
}

// FiatPayoutsResponse from get fiat payouts.
type FiatPayoutsResponse struct {
	Result struct {
		Rows []FiatPayoutRecord `json:"rows"`
	} `json:"result"`
}
