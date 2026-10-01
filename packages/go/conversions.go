package nowpayments

import (
	"fmt"
	"net/url"
	"strings"
)

// CreateConversion creates a conversion within custody account. Requires JWT.
func (c *Client) CreateConversion(params CreateConversionParams, jwtToken string) (interface{}, error) {
	if strings.TrimSpace(jwtToken) == "" {
		return nil, &NowPaymentsError{Message: "JWT token is required for CreateConversion. Call GetAuthToken first."}
	}
	var result interface{}
	if err := c.post("/v1/conversion", params, &result, jwtToken); err != nil {
		return nil, err
	}
	return result, nil
}

// GetConversionStatus returns conversion status. Requires JWT.
func (c *Client) GetConversionStatus(conversionID, jwtToken string) (interface{}, error) {
	if strings.TrimSpace(jwtToken) == "" {
		return nil, &NowPaymentsError{Message: "JWT token is required for GetConversionStatus. Call GetAuthToken first."}
	}
	var result interface{}
	if err := c.get("/v1/conversion/"+url.PathEscape(conversionID), nil, &result, jwtToken); err != nil {
		return nil, err
	}
	return result, nil
}

// GetConversionsParams for listing conversions.
type GetConversionsParams struct {
	ID             []int   `json:"id,omitempty"`
	Status         []string `json:"status,omitempty"`
	FromCurrency   string  `json:"from_currency,omitempty"`
	ToCurrency     string  `json:"to_currency,omitempty"`
	CreatedAtFrom  string  `json:"created_at_from,omitempty"`
	CreatedAtTo    string  `json:"created_at_to,omitempty"`
	Limit          int     `json:"limit,omitempty"`
	Offset         int     `json:"offset,omitempty"`
	Order          string  `json:"order,omitempty"` // ASC or DESC
}

// GetConversions lists conversions. JWT optional.
func (c *Client) GetConversions(params *GetConversionsParams, jwtToken string) (interface{}, error) {
	q := url.Values{}
	if params != nil {
		for _, id := range params.ID {
			q.Add("id", fmt.Sprintf("%d", id))
		}
		for _, s := range params.Status {
			q.Add("status", s)
		}
		if params.FromCurrency != "" {
			q.Set("from_currency", params.FromCurrency)
		}
		if params.ToCurrency != "" {
			q.Set("to_currency", params.ToCurrency)
		}
		if params.CreatedAtFrom != "" {
			q.Set("created_at_from", params.CreatedAtFrom)
		}
		if params.CreatedAtTo != "" {
			q.Set("created_at_to", params.CreatedAtTo)
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
	path := "/v1/conversion"
	if len(q) > 0 {
		path += "?" + q.Encode()
	}
	var result interface{}
	if err := c.get(path, nil, &result, jwtToken); err != nil {
		return nil, err
	}
	return result, nil
}
