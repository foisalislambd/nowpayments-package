package nowpayments

import (
	"fmt"
	"net/url"
	"strings"
)

// CreatePayout creates a mass payout. Requires JWT (call GetAuthToken first).
func (c *Client) CreatePayout(params CreatePayoutParams, jwtToken string) (*CreatePayoutResponse, error) {
	if strings.TrimSpace(jwtToken) == "" {
		return nil, &NowPaymentsError{Message: "JWT token is required for CreatePayout. Call GetAuthToken first."}
	}
	var result CreatePayoutResponse
	if err := c.post("/v1/payout", params, &result, jwtToken); err != nil {
		return nil, err
	}
	return &result, nil
}

// VerifyPayout verifies payout with 2FA code. Requires JWT.
// API may return plain text "OK" or JSON {"result":"ok"}.
func (c *Client) VerifyPayout(payoutID, verificationCode, jwtToken string) (string, error) {
	if strings.TrimSpace(jwtToken) == "" {
		return "", &NowPaymentsError{Message: "JWT token is required for VerifyPayout. Call GetAuthToken first."}
	}
	body := map[string]string{"verification_code": verificationCode}
	var result interface{}
	if err := c.post("/v1/payout/"+url.PathEscape(payoutID)+"/verify", body, &result, jwtToken); err != nil {
		return "", err
	}
	switch v := result.(type) {
	case string:
		return strings.Trim(v, `"`), nil
	case map[string]interface{}:
		if s, ok := v["result"].(string); ok {
			return s, nil
		}
	}
	return fmt.Sprintf("%v", result), nil
}

// GetPayoutStatus returns payout status.
func (c *Client) GetPayoutStatus(payoutID, jwtToken string) (interface{}, error) {
	var result interface{}
	if err := c.get("/v1/payout/"+url.PathEscape(payoutID), nil, &result, jwtToken); err != nil {
		return nil, err
	}
	return result, nil
}

// GetPayoutsParams for listing payouts.
type GetPayoutsParams struct {
	BatchID  string `json:"batch_id,omitempty"`
	Status   string `json:"status,omitempty"`
	OrderBy  string `json:"order_by,omitempty"`
	Order    string `json:"order,omitempty"`
	DateFrom string `json:"date_from,omitempty"`
	DateTo   string `json:"date_to,omitempty"`
	Limit    int    `json:"limit,omitempty"`
	Page     int    `json:"page,omitempty"`
}

// GetPayouts returns list of payouts.
func (c *Client) GetPayouts(params *GetPayoutsParams) (interface{}, error) {
	q := url.Values{}
	if params != nil {
		if params.BatchID != "" {
			q.Set("batch_id", params.BatchID)
		}
		if params.Status != "" {
			q.Set("status", params.Status)
		}
		if params.OrderBy != "" {
			q.Set("order_by", params.OrderBy)
		}
		if params.Order != "" {
			q.Set("order", params.Order)
		}
		if params.DateFrom != "" {
			q.Set("date_from", params.DateFrom)
		}
		if params.DateTo != "" {
			q.Set("date_to", params.DateTo)
		}
		if params.Limit > 0 {
			q.Set("limit", fmt.Sprintf("%d", params.Limit))
		}
		if params.Page > 0 {
			q.Set("page", fmt.Sprintf("%d", params.Page))
		}
	}
	path := "/v1/payout"
	if len(q) > 0 {
		path += "?" + q.Encode()
	}
	var result interface{}
	if err := c.get(path, nil, &result, ""); err != nil {
		return nil, err
	}
	return result, nil
}

// ValidatePayoutAddress validates payout address before creating payout.
func (c *Client) ValidatePayoutAddress(params ValidateAddressParams) (interface{}, error) {
	var result interface{}
	if err := c.post("/v1/payout/validate-address", params, &result, ""); err != nil {
		return nil, err
	}
	return result, nil
}

// GetPayoutFee estimates network fee for a payout.
func (c *Client) GetPayoutFee(currency string, amount float64) (interface{}, error) {
	if strings.TrimSpace(currency) == "" {
		return nil, &NowPaymentsError{Message: `Currency is required (e.g. "btc", "eth")`}
	}
	q := url.Values{}
	q.Set("currency", currency)
	q.Set("amount", fmt.Sprintf("%v", amount))
	path := "/v1/payout/fee?" + q.Encode()
	var result interface{}
	if err := c.get(path, nil, &result, ""); err != nil {
		return nil, err
	}
	return result, nil
}

// CancelPayout cancels a scheduled payout. Requires JWT.
func (c *Client) CancelPayout(payoutID, jwtToken string) error {
	if strings.TrimSpace(jwtToken) == "" {
		return &NowPaymentsError{Message: "JWT token is required for CancelPayout. Call GetAuthToken first."}
	}
	body := map[string]string{"payout_id": payoutID}
	return c.post("/v1/payout/w_id/cancel", body, nil, jwtToken)
}
