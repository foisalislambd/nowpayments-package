package nowpayments

import (
	"fmt"
	"net/url"
	"strings"
)

// GetEstimatePrice returns estimated price in crypto for a fiat amount.
func (c *Client) GetEstimatePrice(params EstimateParams) (*EstimatePriceResponse, error) {
	q := url.Values{}
	q.Set("amount", fmt.Sprintf("%g", params.Amount))
	q.Set("currency_from", params.CurrencyFrom)
	q.Set("currency_to", params.CurrencyTo)
	var result EstimatePriceResponse
	if err := c.do("GET", "/v1/estimate?"+q.Encode(), nil, &result, ""); err != nil {
		return nil, err
	}
	return &result, nil
}

// GetMinAmount returns minimum payment amount for currency pair.
func (c *Client) GetMinAmount(params MinAmountParams) (*MinAmountResponse, error) {
	q := url.Values{}
	q.Set("currency_from", params.CurrencyFrom)
	q.Set("currency_to", params.CurrencyTo)
	if params.FiatEquivalent != nil {
		switch v := params.FiatEquivalent.(type) {
		case bool:
			q.Set("fiat_equivalent", boolStr(v))
		case string:
			q.Set("fiat_equivalent", v)
		}
	}
	if params.IsFixedRate != nil {
		q.Set("is_fixed_rate", boolStr(*params.IsFixedRate))
	}
	if params.IsFeePaidByUser != nil {
		q.Set("is_fee_paid_by_user", boolStr(*params.IsFeePaidByUser))
	}
	var result MinAmountResponse
	if err := c.do("GET", "/v1/min-amount?"+q.Encode(), nil, &result, ""); err != nil {
		return nil, err
	}
	return &result, nil
}

// CreatePayment creates a new payment. Returns address + amount for customer to pay.
func (c *Client) CreatePayment(params CreatePaymentParams) (*Payment, error) {
	body := params
	if body.FixedRate != nil && body.IsFixedRate == nil {
		body.IsFixedRate = body.FixedRate
	}
	// Never send deprecated fixed_rate field — API expects is_fixed_rate
	body.FixedRate = nil
	var result Payment
	if err := c.post("/v1/payment", body, &result, ""); err != nil {
		return nil, err
	}
	return &result, nil
}

// GetPaymentStatus returns payment status by ID.
func (c *Client) GetPaymentStatus(paymentID interface{}) (*Payment, error) {
	idStr := strings.TrimSpace(fmt.Sprintf("%v", paymentID))
	if idStr == "" {
		return nil, &NowPaymentsError{Message: "Payment ID is required"}
	}
	var result Payment
	if err := c.get("/v1/payment/"+url.PathEscape(idStr), nil, &result, ""); err != nil {
		return nil, err
	}
	return &result, nil
}

// GetPayments returns paginated list of payments. JWT recommended per API docs.
func (c *Client) GetPayments(params *ListPaymentsParams, jwtToken string) (*PaymentsListResponse, error) {
	q := url.Values{}
	if params != nil {
		if params.Limit > 0 {
			q.Set("limit", fmt.Sprintf("%d", params.Limit))
		}
		if params.Page > 0 {
			q.Set("page", fmt.Sprintf("%d", params.Page))
		}
		if params.SortBy != "" {
			q.Set("sortBy", params.SortBy)
		}
		if params.OrderBy != "" {
			q.Set("orderBy", params.OrderBy)
		}
		if params.DateFrom != "" {
			q.Set("dateFrom", params.DateFrom)
		}
		if params.DateTo != "" {
			q.Set("dateTo", params.DateTo)
		}
	}
	path := "/v1/payment/"
	if len(q) > 0 {
		path += "?" + q.Encode()
	}
	var result PaymentsListResponse
	if err := c.do("GET", path, nil, &result, jwtToken); err != nil {
		return nil, err
	}
	return &result, nil
}

// UpdatePaymentEstimate updates payment estimate (call before expiration).
func (c *Client) UpdatePaymentEstimate(paymentID interface{}) (*UpdatePaymentEstimateResponse, error) {
	idStr := strings.TrimSpace(fmt.Sprintf("%v", paymentID))
	if idStr == "" {
		return nil, &NowPaymentsError{Message: "Payment ID is required"}
	}
	var result UpdatePaymentEstimateResponse
	if err := c.post("/v1/payment/"+url.PathEscape(idStr)+"/update-merchant-estimate", nil, &result, ""); err != nil {
		return nil, err
	}
	return &result, nil
}
